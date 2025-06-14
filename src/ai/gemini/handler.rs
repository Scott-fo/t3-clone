use anyhow::{Context, Result, anyhow};
use futures_util::StreamExt;
use reqwest::{Client, RequestBuilder};
use reqwest_eventsource::{Event, EventSource};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    ai::{
        gemini::{model::GeminiModel, request::*},
        handler::{StreamResult, create_title_prompt, done, send_error, send_text_delta},
    },
    models::message::Message,
    services::sse_manager::SseManager,
};

const GOOGLE_SSE_BASE: &str = "https://generativelanguage.googleapis.com/v1beta/models";

pub async fn stream(
    api_key: SecretString,
    sse: Arc<SseManager>,
    user_id: String,
    chat_id: String,
    model: GeminiModel,
    messages: Vec<Message>,
) -> Result<Option<StreamResult>> {
    let req_body = GeminiRequest::chat(&messages);
    let url = format!(
        "{}/{model}:streamGenerateContent?alt=sse&key={}",
        GOOGLE_SSE_BASE,
        api_key.expose_secret(),
        model = model.to_string()
    );

    let http_req: RequestBuilder = Client::new()
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&req_body);

    let mut es = EventSource::new(http_req).context("Gemini SSE connect")?;

    let msg_id = Uuid::new_v4().to_string();
    let mut full_text = String::new();

    while let Some(ev) = es.next().await {
        match ev {
            Ok(Event::Open) => info!("Gemini SSE opened"),
            Ok(Event::Message(msg)) => {
                if msg.data.trim() == "[DONE]" {
                    break;
                }
                if msg.data.trim().is_empty() {
                    continue;
                }

                let payload: GeminiSsePayload = match serde_json::from_str(&msg.data) {
                    Ok(p) => p,
                    Err(e) => {
                        warn!(error = %e, raw = msg.data, "Gemini JSON parse error");
                        continue;
                    }
                };

                let (delta, stop, fail) = parse_payload(payload);

                if let Some(reason) = fail {
                    send_error(&sse, &user_id, &chat_id, &reason).await;
                    es.close();
                    return Err(anyhow!("Gemini stopped: {reason}"));
                }

                if !delta.is_empty() {
                    full_text.push_str(&delta);
                    send_text_delta(&sse, &user_id, &chat_id, &delta).await;
                }

                if stop {
                    break;
                }
            }
            Err(e) => {
                send_error(&sse, &user_id, &chat_id, &e.to_string()).await;
                es.close();
                return Err(anyhow!(e));
            }
        }
    }

    if full_text.is_empty() {
        warn!("Gemini stream ended with no text");
        return Ok(None);
    }

    done(&sse, &user_id, &chat_id, &msg_id).await;

    Ok(Some(StreamResult {
        msg_id,
        content: full_text,
        reasoning: None,
    }))
}

pub async fn generate_title(
    api_key: &SecretString,
    first_user_message: &str,
    model: GeminiModel,
) -> Result<String> {
    let req_body = GeminiRequest::prompt(create_title_prompt(first_user_message));

    let url = format!(
        "{}/{model}:generateContent?key={}",
        GOOGLE_SSE_BASE,
        api_key.expose_secret(),
        model = model.to_string()
    );

    let payload: GeminiSsePayload = Client::new()
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&req_body)
        .send()
        .await
        .context("Google title request failed")?
        .error_for_status()
        .context("Google title request HTTP error")?
        .json()
        .await
        .context("Google title JSON decode failed")?;

    let title = payload
        .candidates
        .into_iter()
        .flat_map(|c| c.content)
        .flat_map(|c| c.parts)
        .filter_map(|p| p.text)
        .collect::<String>()
        .trim()
        .to_owned();

    if title.is_empty() {
        Err(anyhow!("Google API returned empty title"))
    } else {
        Ok(title)
    }
}

#[derive(Debug, Deserialize)]
struct GeminiSsePayload {
    #[serde(default)]
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: Option<Content>,
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Content {
    #[serde(default)]
    parts: Vec<Part>,
}

#[derive(Debug, Deserialize)]
struct Part {
    text: Option<String>,
}

fn parse_payload(payload: GeminiSsePayload) -> (String, bool, Option<String>) {
    let mut delta = String::new();
    let mut stop = false;
    let mut fail: Option<String> = None;

    for cand in payload.candidates {
        if let Some(content) = cand.content {
            for part in content.parts {
                if let Some(t) = part.text {
                    delta.push_str(&t);
                }
            }
        }
        match cand.finish_reason.as_deref() {
            Some("STOP") => stop = true,
            Some(r) => fail = Some(r.to_owned()),
            None => {}
        }
    }
    (delta, stop, fail)
}

use anyhow::{Context, Result, anyhow};
use futures_util::StreamExt;
use reqwest::{Client, RequestBuilder};
use reqwest_eventsource::{Event, EventSource};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    models::message::Message,
    services::sse_manager::{EventType, SseManager, SseMessage},
};

use super::handler::create_title_prompt;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum GeminiModel {
    #[serde(rename = "gemini-2.5-pro-preview-06-05")]
    Pro25,
    #[serde(rename = "gemini-2.5-flash-preview-05-20")]
    Flash25,
    #[serde(rename = "gemini-2.0-flash")]
    Flash20,
}

impl ToString for GeminiModel {
    fn to_string(&self) -> String {
        match self {
            Self::Pro25 => "gemini-2.5-pro-preview-06-05".into(),
            Self::Flash25 => "gemini-2.5-flash-preview-05-20".into(),
            Self::Flash20 => "gemini-2.0-flash".into(),
        }
    }
}

#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiMessage>,
}

#[derive(Debug, Serialize)]
struct GeminiMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize)]
struct GeminiPart {
    text: String,
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

pub async fn stream_gemini_response(
    api_key: SecretString,
    sse_manager: Arc<SseManager>,
    user_id: String,
    chat_id: String,
    model: String,
    messages: Vec<Message>,
) -> Result<Option<super::handler::StreamResult>> {
    process_stream(
        &api_key,
        &sse_manager,
        &user_id,
        &chat_id,
        &model,
        &messages,
    )
    .await
}

async fn process_stream(
    api_key: &SecretString,
    sse_manager: &SseManager,
    user_id: &str,
    chat_id: &str,
    model: &str,
    messages: &Vec<Message>,
) -> Result<Option<super::handler::StreamResult>> {
    let mut contents: Vec<GeminiMessage> = Vec::with_capacity(messages.len());
    for m in messages {
        let mapped_role = match m.role.as_str() {
            "assistant" => "model",
            "user" => "user",
            other => {
                warn!(%other, "Unknown role for Gemini. defaulting to user");
                "user"
            }
        };

        contents.push(GeminiMessage {
            role: Some(mapped_role.to_owned()),
            parts: vec![GeminiPart {
                text: m.body.clone(),
            }],
        });
    }

    let req_body = GeminiRequest { contents };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?alt=sse&key={}",
        model,
        api_key.expose_secret()
    );

    let client = Client::new();
    let http_req: RequestBuilder = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&req_body);

    let mut es = EventSource::new(http_req).context("Failed to connect to Gemini SSE")?;

    let msg_id = Uuid::new_v4().to_string();
    let mut full_text = String::new();

    while let Some(ev) = es.next().await {
        match ev {
            Ok(Event::Open) => info!("Gemini SSE connection opened"),
            Ok(Event::Message(msg)) => {
                tracing::info!(raw = %msg.data, "Gemini raw SSE");
                if msg.data.trim().is_empty() {
                    continue;
                }

                if msg.data.trim() == "[DONE]" {
                    break;
                }

                let parsed: Result<GeminiSsePayload, _> = serde_json::from_str(&msg.data);
                let payload = match parsed {
                    Ok(p) => p,
                    Err(e) => {
                        warn!(error = %e, raw = msg.data, "Gemini JSON parse error");
                        continue;
                    }
                };

                let mut delta_txt = String::new();
                let mut stop_signal = false;
                let mut fail_reason: Option<String> = None;

                for cand in payload.candidates {
                    if let Some(content) = cand.content {
                        for part in content.parts {
                            if let Some(t) = part.text {
                                delta_txt.push_str(&t);
                            }
                        }
                    }

                    match cand.finish_reason.as_deref() {
                        Some("STOP") => stop_signal = true,
                        Some(r) => fail_reason = Some(r.to_owned()),
                        None => {}
                    }
                }

                if delta_txt.is_empty() {
                    continue;
                }

                full_text.push_str(&delta_txt);

                if let Some(r) = fail_reason {
                    let err_payload = json!({
                        "chat_id": chat_id,
                        "error": format!("Gemini stopped: {r}"),
                    });
                    sse_manager
                        .send_to_user(
                            user_id,
                            SseMessage {
                                event_type: EventType::Err,
                                data: Some(err_payload),
                            },
                        )
                        .await;

                    es.close();
                    return Err(anyhow!("Gemini stream failed: {r}"));
                }

                let chunk_payload = json!({
                    "chat_id": chat_id,
                    "chunk": delta_txt,
                });

                sse_manager
                    .send_to_user(
                        user_id,
                        SseMessage {
                            event_type: EventType::Chunk,
                            data: Some(chunk_payload),
                        },
                    )
                    .await;

                if stop_signal {
                    break;
                }
            }
            Err(e) => {
                warn!(error = %e, "Gemini SSE stream error");

                let err_payload = json!({
                    "chat_id": chat_id,
                    "error": e.to_string(),
                });

                sse_manager
                    .send_to_user(
                        user_id,
                        SseMessage {
                            event_type: EventType::Err,
                            data: Some(err_payload),
                        },
                    )
                    .await;

                es.close();
                return Err(anyhow!("Gemini SSE stream error: {}", e));
            }
        }
    }

    if full_text.is_empty() {
        warn!("Gemini stream ended but no content was received.");
        return Ok(None);
    }

    let done_payload = json!({
        "chat_id": chat_id,
        "msg_id": msg_id,
    });

    sse_manager
        .send_to_user(
            user_id,
            SseMessage {
                event_type: EventType::Done,
                data: Some(done_payload),
            },
        )
        .await;

    Ok(Some(super::handler::StreamResult {
        msg_id,
        content: full_text,
        reasoning: None,
    }))
}

pub async fn generate_title(
    api_key: &SecretString,
    first_message: &str,
    model: &str,
) -> Result<String> {
    let req_body = GeminiRequest {
        contents: vec![GeminiMessage {
            role: Some("user".into()),
            parts: vec![GeminiPart {
                text: create_title_prompt(first_message),
            }],
        }],
    };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model,
        api_key.expose_secret()
    );

    let payload: GeminiSsePayload = Client::new()
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&req_body)
        .send()
        .await
        .context("Google title request failed")?
        .error_for_status()
        .context("Google title request returned error")?
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

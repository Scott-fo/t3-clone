use std::sync::Arc;

use anyhow::{Context, Result};
use futures_util::StreamExt;
use reqwest::{
    Client, RequestBuilder,
    header::{HeaderMap, HeaderValue},
};
use reqwest_eventsource::{Event, EventSource};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

use super::{model::OpenRouterModel, request::OpenRouterRequest};
use crate::{
    ai::handler::{StreamResult, create_title_prompt, done, send_error, send_text_delta},
    models::message::Message,
    services::sse_manager::SseManager,
};

const BASE: &str = "https://openrouter.ai/api/v1/chat/completions";

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: AssistantMessage,
}

#[derive(Debug, Deserialize)]
pub struct AssistantMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct CompletionResponse {
    pub id: String,
    pub choices: Vec<Choice>,
}

pub async fn generate_title(
    api_key: &SecretString,
    first_body: &str,
    model: OpenRouterModel,
) -> Result<String> {
    let prompt = create_title_prompt(first_body);
    let model = model.to_string();
    let req = OpenRouterRequest::prompt(&model, &prompt);

    let client = Client::new();
    let resp: CompletionResponse = client
        .post(BASE)
        .bearer_auth(api_key.expose_secret())
        .json(&req)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(resp
        .choices
        .first()
        .context("no choices in OpenRouter response")?
        .message
        .content
        .trim()
        .to_owned())
}

#[derive(Debug, Deserialize)]
pub struct Delta {
    pub content: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChunkChoice {
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StreamChunk {
    pub id: String,
    pub choices: Vec<ChunkChoice>,
}

pub async fn stream(
    api_key: SecretString,
    sse: Arc<SseManager>,
    user_id: String,
    chat_id: String,
    model: OpenRouterModel,
    history: Vec<Message>,
) -> Result<Option<StreamResult>> {
    let model = model.to_string();
    let req_body = OpenRouterRequest::chat(&model, &history, true);

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key.expose_secret()))?,
    );

    let http_req: RequestBuilder = Client::new().post(BASE).headers(headers).json(&req_body);

    let mut es = EventSource::new(http_req).context("OpenRouter SSE connect")?;

    let msg_id = Uuid::new_v4().to_string();
    let mut full_text = String::new();

    while let Some(ev) = es.next().await {
        match ev {
            Ok(Event::Open) => info!("OpenRouter SSE opened"),
            Ok(Event::Message(msg)) => {
                let data = msg.data.trim();
                if data == "[DONE]" {
                    break;
                }
                if data.is_empty() {
                    continue;
                }

                let chunk: StreamChunk = serde_json::from_str(data)?;
                if let Some(choice) = chunk.choices.first() {
                    if let Some(content) = &choice.delta.content {
                        send_text_delta(&sse, &user_id, &chat_id, content).await;
                        full_text.push_str(content);
                    }

                    if matches!(&choice.finish_reason, Some(r) if r == "stop") {
                        break;
                    }
                }
            }
            Err(e) => {
                send_error(&sse, &user_id, &chat_id, &e.to_string()).await;
                return Ok(None);
            }
        }
    }

    done(&sse, &user_id, &chat_id, &msg_id).await;

    Ok(Some(StreamResult {
        msg_id,
        content: full_text,
        reasoning: None,
    }))
}

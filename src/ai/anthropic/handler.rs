use std::sync::Arc;

use anyhow::{Context, Result};
use futures_util::StreamExt;
use reqwest::{
    Client, RequestBuilder,
    header::{HeaderMap, HeaderValue},
};
use reqwest_eventsource::{Event, EventSource};
use secrecy::{ExposeSecret, SecretString};
use serde_json::Value;
use tracing::info;
use uuid::Uuid;

use super::{model::AnthropicModel, request::AnthropicRequest};
use crate::{
    ai::handler::{StreamResult, create_title_prompt, done, send_error, send_text_delta},
    models::message::Message,
    services::sse_manager::SseManager,
};

const BASE: &str = "https://api.anthropic.com/v1/messages";
const VERSION: &str = "2023-06-01";

pub async fn generate_title(
    api_key: &SecretString,
    first_body: &str,
    model: AnthropicModel,
) -> Result<String> {
    let model = model.to_string();
    let prompt = create_title_prompt(first_body);
    let req = AnthropicRequest::prompt(&model, &prompt);

    let mut headers = HeaderMap::new();

    headers.insert("x-api-key", HeaderValue::from_str(api_key.expose_secret())?);
    headers.insert("anthropic-version", HeaderValue::from_static(VERSION));

    let client = Client::new();
    let resp: Value = client
        .post(BASE)
        .headers(headers)
        .json(&req)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let text = resp["content"][0]["text"]
        .as_str()
        .context("bad anthropic response")?;
    Ok(text.trim().to_owned())
}

pub async fn stream(
    api_key: SecretString,
    sse: Arc<SseManager>,
    user_id: String,
    chat_id: String,
    model: AnthropicModel,
    history: Vec<Message>,
) -> Result<Option<StreamResult>> {
    let model = model.to_string();
    let req_body = AnthropicRequest::chat(&model, &history, true);

    let mut headers = HeaderMap::new();
    headers.insert("x-api-key", HeaderValue::from_str(api_key.expose_secret())?);
    headers.insert("anthropic-version", HeaderValue::from_static(VERSION));

    let http_req: RequestBuilder = Client::new().post(BASE).headers(headers).json(&req_body);

    let mut es = EventSource::new(http_req).context("Anthropic SSE connect")?;

    let msg_id = Uuid::new_v4().to_string();
    let mut full_text = String::new();

    while let Some(ev) = es.next().await {
        match ev {
            Ok(Event::Open) => info!("Anthropic SSE opened"),
            Ok(Event::Message(msg)) => {
                if msg.data.trim() == "[DONE]" {
                    break;
                }
                if msg.data.trim().is_empty() {
                    continue;
                }

                let v: Value = serde_json::from_str(&msg.data)?;
                match v["type"].as_str() {
                    Some("content_block_delta") => {
                        if let Some(t) = v["delta"]["text"].as_str() {
                            send_text_delta(&sse, &user_id, &chat_id, t).await;
                            full_text.push_str(t);
                        }
                    }
                    Some("message_stop") => break,
                    _ => {}
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

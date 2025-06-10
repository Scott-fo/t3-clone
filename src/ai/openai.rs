use anyhow::{Context, Result, anyhow};
use futures_util::StreamExt;
use reqwest::Client;
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, sync::Arc};
use tracing::{info, warn};

use crate::services::sse_manager::{SseManager, SseMessage};

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    input: String,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_response_id: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum StreamEvent {
    #[serde(rename = "response.in_progress")]
    ResponseInProgress { response: ResponseObject },
    #[serde(rename = "response.completed")]
    ResponseCompleted { response: ResponseObject },
    #[serde(rename = "response.failed")]
    ResponseFailed { response: ResponseObject },
    #[serde(rename = "response.output_text.delta")]
    ResponseOutputTextDelta { item_id: String, delta: String },
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug)]
struct ResponseObject {
    id: String,
    output: Option<Vec<MessageOutput>>,
    error: Option<OpenAIError>,
}

#[derive(Deserialize, Debug)]
struct MessageOutput {
    #[serde(rename = "type")]
    output_type: String,
    content: Vec<ContentPart>,
}

#[derive(Deserialize, Debug)]
struct ContentPart {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Deserialize, Debug)]
struct OpenAIError {
    message: String,
}

pub async fn stream_openai_response(
    sse_manager: Arc<SseManager>,
    user_id: String,
    chat_id: String,
    prompt: String,
    model: String,
    previous_response_id: Option<String>,
) -> Result<Option<(String, String)>> {
    process_stream(
        &sse_manager,
        &user_id,
        &chat_id,
        &prompt,
        &model,
        previous_response_id,
    )
    .await
}

async fn process_stream(
    sse_manager: &SseManager,
    user_id: &str,
    chat_id: &str,
    prompt: &str,
    model: &str,
    previous_response_id: Option<String>,
) -> Result<Option<(String, String)>> {
    let api_key = env::var("OPENAI_API_KEY").context("OPENAI_API_KEY must be set")?;
    let client = Client::new();

    let request_body = OpenAIRequest {
        model: model.to_string(),
        input: prompt.to_string(),
        stream: true,
        previous_response_id,
    };

    let request = client
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(api_key)
        .json(&request_body);

    let mut es = EventSource::new(request).context("Failed to create event source")?;

    while let Some(event) = es.next().await {
        match event {
            Ok(Event::Open) => {
                info!("OpenAI connection opened");
            }
            Ok(Event::Message(message)) => {
                let data = &message.data;
                let event: StreamEvent = match serde_json::from_str(data) {
                    Ok(event) => event,
                    Err(e) => {
                        warn!(error = %e, data, "Failed to parse OpenAI JSON chunk");
                        continue;
                    }
                };

                match event {
                    StreamEvent::ResponseOutputTextDelta { item_id, delta } => {
                        let chunk_payload = json!({
                            "chat_id": chat_id,
                            "chunk": delta,
                        });

                        sse_manager
                            .send_to_user(
                                user_id,
                                SseMessage {
                                    event_type: "chat-stream-chunk".to_string(),
                                    data: Some(chunk_payload),
                                },
                            )
                            .await;
                    }
                    StreamEvent::ResponseCompleted { response } => {
                        let msg_id = Some(response.id);

                        let final_content = response
                            .output
                            .as_deref()
                            .unwrap_or(&[])
                            .iter()
                            .find(|o| o.output_type == "message")
                            .and_then(|msg| {
                                msg.content.iter().find(|c| c.content_type == "output_text")
                            })
                            .map(|content| content.text.clone())
                            .unwrap_or_default();

                        let done_payload = json!({
                            "chat_id": chat_id,
                            "msg_id": msg_id.clone().unwrap_or_default(),
                        });
                        sse_manager
                            .send_to_user(
                                user_id,
                                SseMessage {
                                    event_type: "chat-stream-done".to_string(),
                                    data: Some(done_payload),
                                },
                            )
                            .await;
                        info!(%chat_id, "OpenAI stream completed successfully.");
                        return Ok(Some((msg_id.unwrap_or_default(), final_content)));
                    }
                    StreamEvent::ResponseFailed { response } => {
                        let error_msg = response
                            .error
                            .map(|e| e.message)
                            .unwrap_or_else(|| "Unknown error".to_string());

                        let error_payload = json!({
                            "chat_id": chat_id,
                            "error": error_msg.clone(),
                        });

                        sse_manager
                            .send_to_user(
                                &user_id,
                                SseMessage {
                                    event_type: "chat-stream-error".to_string(),
                                    data: Some(error_payload),
                                },
                            )
                            .await;

                        return Err(anyhow!("OpenAI failed: {}", error_msg));
                    }
                    StreamEvent::ResponseInProgress { .. } => { /* Do nothing */ }
                    StreamEvent::Unknown => {
                        warn!(data, "Received an unknown event type from OpenAI.");
                    }
                }
            }
            Err(e) => {
                warn!(error = %e, "EventSource stream error.");
                es.close();
                return Err(anyhow!("EventSource stream error: {}", e));
            }
        }
    }

    warn!("Stream ended without a 'completed' or 'failed' event.");
    Ok(None)
}

pub async fn generate_title(first_message: &str) -> Result<String> {
    const TITLE_MODEL: &str = "gpt-4.1-nano";

    let api_key = env::var("OPENAI_API_KEY").context("OPENAI_API_KEY must be set")?;
    let client = Client::new();

    let prompt = format!(
        "Summarize the following message into a short, concise title of 5 words or less, without quotation marks: \"{}\"",
        first_message
    );

    let request_body = OpenAIRequest {
        model: TITLE_MODEL.to_string(),
        input: prompt,
        stream: false,
        previous_response_id: None,
    };

    let response = client
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await
        .context("Failed to send request to OpenAI for title generation")?;

    let response = response
        .error_for_status()
        .context("OpenAI API returned an error status")?;

    let response_object: ResponseObject = response
        .json()
        .await
        .context("Failed to deserialize OpenAI response object")?;

    let title = response_object
        .output
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .find(|o| o.output_type == "message")
        .and_then(|msg| msg.content.iter().find(|c| c.content_type == "output_text"))
        .map(|content| content.text.clone())
        .context("OpenAI response did not contain valid output text")?;

    Ok(title)
}

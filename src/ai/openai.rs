use anyhow::{Context, Result, anyhow};
use futures_util::StreamExt;
use reqwest::Client;
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, sync::Arc};
use tracing::{info, warn};

use crate::services::sse_manager::{EventType, SseManager, SseMessage};

use super::reasoning::{EffortLevel, Reasoning};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "model")]
pub enum OpenAIRequest {
    #[serde(rename = "gpt-4o")]
    Gpt4o {
        input: String,
        stream: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        previous_response_id: Option<String>,
        instructions: Option<String>,
    },
    #[serde(rename = "gpt-4.1")]
    Gpt41 {
        input: String,
        stream: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        previous_response_id: Option<String>,
        instructions: Option<String>,
    },
    #[serde(rename = "gpt-4.1-mini")]
    Gpt41Mini {
        input: String,
        stream: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        previous_response_id: Option<String>,
        instructions: Option<String>,
    },
    #[serde(rename = "gpt-4.1-nano")]
    Gpt41Nano {
        input: String,
        stream: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        previous_response_id: Option<String>,
        instructions: Option<String>,
    },
    #[serde(rename = "o3-mini")]
    O3Mini {
        input: String,
        stream: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        previous_response_id: Option<String>,
        reasoning: Reasoning,
        instructions: Option<String>,
    },
    #[serde(rename = "o4-mini")]
    O4Mini {
        input: String,
        stream: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        previous_response_id: Option<String>,
        reasoning: Reasoning,
        instructions: Option<String>,
    },
    #[serde(rename = "o3")]
    O3 {
        input: String,
        stream: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        previous_response_id: Option<String>,
        reasoning: Reasoning,
        instructions: Option<String>,
    },
}

impl OpenAIRequest {
    pub fn new_from_str(
        model: &str,
        input: String,
        stream: bool,
        previous_response_id: Option<String>,
        effort: Option<EffortLevel>,
        instructions: Option<String>,
    ) -> Result<Self> {
        match model {
            "gpt-4o" => Ok(Self::Gpt4o {
                input,
                stream,
                previous_response_id,
                instructions,
            }),
            "gpt-4.1" => Ok(Self::Gpt41 {
                input,
                stream,
                previous_response_id,
                instructions,
            }),
            "gpt-4.1-mini" => Ok(Self::Gpt41Mini {
                input,
                stream,
                previous_response_id,
                instructions,
            }),
            "gpt-4.1-nano" => Ok(Self::Gpt41Nano {
                input,
                stream,
                previous_response_id,
                instructions,
            }),
            "o4-mini" => {
                let effort =
                    effort.ok_or_else(|| anyhow!("`o4-mini` requires a reasoning field"))?;
                Ok(Self::O4Mini {
                    input,
                    stream,
                    previous_response_id,
                    reasoning: Reasoning::new(effort),
                    instructions,
                })
            }
            "o3-mini" => {
                let effort =
                    effort.ok_or_else(|| anyhow!("`o3-mini` requires a reasoning field"))?;
                Ok(Self::O3Mini {
                    input,
                    stream,
                    previous_response_id,
                    reasoning: Reasoning::new(effort),
                    instructions,
                })
            }
            "o3" => {
                let effort = effort.ok_or_else(|| anyhow!("`o3` requires a reasoning field"))?;
                Ok(Self::O3 {
                    input,
                    stream,
                    previous_response_id,
                    reasoning: Reasoning::new(effort),
                    instructions,
                })
            }
            other => Err(anyhow!("unknown/unsupported model: {other}")),
        }
    }
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

    #[serde(rename = "response.reasoning_summary_text.delta")]
    ResponseReasoningSummaryTextDelta { item_id: String, delta: String },

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

    #[serde(default)]
    content: Vec<ContentPart>,

    #[serde(default)]
    summary: Vec<SummaryPart>,
}

#[derive(Deserialize, Debug)]
struct SummaryPart {
    #[serde(rename = "type")]
    summary_type: String,
    text: String,
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

pub struct StreamResult {
    pub msg_id: String,
    pub content: String,
    pub reasoning: Option<String>,
}

pub async fn stream_openai_response(
    sse_manager: Arc<SseManager>,
    user_id: String,
    chat_id: String,
    prompt: String,
    model: String,
    previous_response_id: Option<String>,
    reasoning: Option<EffortLevel>,
) -> Result<Option<StreamResult>> {
    process_stream(
        &sse_manager,
        &user_id,
        &chat_id,
        &prompt,
        &model,
        previous_response_id,
        reasoning,
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
    effort: Option<EffortLevel>,
) -> Result<Option<StreamResult>> {
    let api_key = env::var("OPENAI_API_KEY").context("OPENAI_API_KEY must be set")?;
    let client = Client::new();

    let instructions = "All code that you generate MUST be generated so that it is correctly rendered inside of a <code> block. Keep decoration in text to a minimum, just respond with clear information, in markdown format.";

    let request_body = OpenAIRequest::new_from_str(
        model,
        prompt.to_string(),
        true,
        previous_response_id,
        effort,
        Some(instructions.to_string()),
    )?;

    let request = client
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(api_key)
        .json(&request_body);

    let mut es = EventSource::new(request).context("Failed to create event source")?;
    let mut reasoning_final: Option<String> = None;

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
                                    event_type: EventType::Chunk,
                                    data: Some(chunk_payload),
                                },
                            )
                            .await;
                    }
                    StreamEvent::ResponseCompleted { response } => {
                        let outputs = response.output.as_deref().unwrap_or(&[]);

                        let maybe_msg_output = outputs.iter().find(|o| o.output_type == "message");

                        if reasoning_final.is_none() {
                            if let Some(summary_text) = outputs
                                .iter()
                                .find(|o| o.output_type == "reasoning")
                                .map(|o| {
                                    o.summary
                                        .iter()
                                        .filter(|p| p.summary_type == "summary_text")
                                        .map(|p| p.text.as_str())
                                        .collect::<Vec<&str>>()
                                        .join("\n\n")
                                })
                            {
                                if !summary_text.is_empty() {
                                    reasoning_final = Some(summary_text);
                                }
                            }
                        }

                        if let Some(msg) = maybe_msg_output {
                            let final_content = msg
                                .content
                                .iter()
                                .find(|c| c.content_type == "output_text")
                                .map(|c| c.text.clone())
                                .unwrap_or_default();

                            let done_payload = json!({
                                "chat_id": chat_id,
                                "msg_id": response.id,
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

                            if let Some(r) = &reasoning_final {
                                info!(%r, "GOT FINAL REASONING");
                            }

                            return Ok(Some(StreamResult {
                                msg_id: response.id,
                                content: final_content,
                                reasoning: reasoning_final,
                            }));
                        }
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
                                    event_type: EventType::Err,
                                    data: Some(error_payload),
                                },
                            )
                            .await;

                        return Err(anyhow!("OpenAI failed: {}", error_msg));
                    }

                    StreamEvent::ResponseReasoningSummaryTextDelta { item_id: _, delta } => {
                        let chunk_payload = json!({
                            "chat_id": chat_id,
                            "reasoning": delta,
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

                        reasoning_final =
                            Some(format!("{}{}", reasoning_final.unwrap_or_default(), delta));
                    }

                    StreamEvent::ResponseInProgress { .. } => { /* Do nothing */ }
                    StreamEvent::Unknown => {
                        warn!(data, "Received an unknown event type from OpenAI.");
                    }
                }
            }
            Err(e) => {
                warn!(error = %e, "EventSource stream error.");

                let error_payload = json!({
                    "chat_id": chat_id,
                    "error": e.to_string(),
                });

                sse_manager
                    .send_to_user(
                        &user_id,
                        SseMessage {
                            event_type: EventType::Err,
                            data: Some(error_payload),
                        },
                    )
                    .await;

                es.close();
                return Err(anyhow!("EventSource stream error: {}", e));
            }
        }
    }

    warn!("Stream ended without a 'completed' or 'failed' event.");
    Ok(None)
}

pub async fn generate_title(first_message: &str) -> Result<String> {
    let api_key = env::var("OPENAI_API_KEY").context("OPENAI_API_KEY must be set")?;
    let client = Client::new();

    let prompt = format!(
        "Summarize the following message into a short, concise title of 5 words or less, without quotation marks: \"{}\"",
        first_message
    );

    let request_body = OpenAIRequest::Gpt41Nano {
        input: prompt,
        stream: false,
        previous_response_id: None,
        instructions: None,
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

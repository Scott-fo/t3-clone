use anyhow::{Context, Result, anyhow};
use futures_util::StreamExt;
use reqwest::Client;
use reqwest_eventsource::{Event, EventSource};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{info, warn};

use crate::{
    ai::{
        handler::{
            StreamResult, create_title_prompt, done, send_error, send_reasoning_delta,
            send_text_delta,
        },
        openai::request::Turn,
        reasoning::EffortLevel,
    },
    models::message::Message,
    services::sse_manager::SseManager,
};

use super::{
    model::OpenAiModel,
    request::{OpenAiRequest, RESPONSES_URL},
};

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum StreamEvent {
    #[serde(rename = "response.completed")]
    ResponseCompleted { response: ResponseObject },

    #[serde(rename = "response.failed")]
    ResponseFailed { response: ResponseObject },

    #[serde(rename = "response.output_text.delta")]
    ResponseOutputTextDelta { delta: String },

    #[serde(rename = "response.reasoning_summary_text.delta")]
    ResponseReasoningSummaryTextDelta { delta: String },

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

const INSTRUCTIONS: &str = "All code that you generate MUST be generated so that it is correctly rendered inside of a <code> block. Keep decoration in text to a minimum, just respond with clear information, in markdown format. RemarkGFM is used to help parse your output.";

pub async fn stream(
    api_key: SecretString,
    sse_manager: Arc<SseManager>,
    user_id: String,
    chat_id: String,
    model: OpenAiModel,
    reasoning: Option<EffortLevel>,
    messages: Vec<Message>,
) -> Result<Option<StreamResult>> {
    let request_body = OpenAiRequest::chat(
        model,
        build_turns(&messages),
        reasoning,
        Some(INSTRUCTIONS.into()),
    )?;

    let req = Client::new()
        .post(RESPONSES_URL)
        .bearer_auth(api_key.expose_secret())
        .json(&request_body);

    let mut es = EventSource::new(req).context("connect sse")?;
    let mut reasoning_final = String::new();

    while let Some(ev) = es.next().await {
        match ev {
            Ok(Event::Open) => info!("OpenAI SSE opened"),
            Ok(Event::Message(msg)) => {
                let evt: StreamEvent = serde_json::from_str(&msg.data)?;
                match evt {
                    StreamEvent::ResponseOutputTextDelta { delta } => {
                        send_text_delta(&sse_manager, &user_id, &chat_id, &delta).await;
                    }
                    StreamEvent::ResponseReasoningSummaryTextDelta { delta } => {
                        send_reasoning_delta(&sse_manager, &user_id, &chat_id, &delta).await;
                        reasoning_final.push_str(&delta);
                    }
                    StreamEvent::ResponseCompleted { response } => {
                        let outputs = response.output.as_deref().unwrap_or(&[]);

                        if reasoning_final.is_empty() {
                            if let Some(r) = extract_reasoning_summary(outputs) {
                                reasoning_final = r;
                            }
                        }

                        let reasoning_opt =
                            (!reasoning_final.is_empty()).then(|| reasoning_final.clone());

                        if let Some(final_content) = extract_message_text(outputs) {
                            done(&sse_manager, &user_id, &chat_id, &response.id).await;

                            return Ok(Some(super::handler::StreamResult {
                                msg_id: response.id,
                                content: final_content,
                                reasoning: reasoning_opt,
                            }));
                        }
                    }
                    StreamEvent::ResponseFailed { response } => {
                        let e = response.error.map(|e| e.message).unwrap_or_default();
                        send_error(&sse_manager, &user_id, &chat_id, &e).await;
                        return Err(anyhow!("OpenAI failed: {e}"));
                    }
                    StreamEvent::Unknown => warn!("unknown event"),
                }
            }
            Err(e) => {
                send_error(&sse_manager, &user_id, &chat_id, &e.to_string()).await;
                return Err(e.into());
            }
        }
    }

    Ok(None)
}

fn build_turns(history: &[Message]) -> Vec<Turn> {
    history
        .iter()
        .map(|m| Turn {
            role: match m.role.as_str() {
                "assistant" => "assistant",
                _ => "user",
            }
            .into(),
            content: &m.body,
        })
        .collect()
}

pub async fn generate_title(
    api_key: &SecretString,
    first_message: &str,
    model: OpenAiModel,
) -> Result<String> {
    let title_prompt = create_title_prompt(first_message);
    let request_body = OpenAiRequest::prompt(model, &title_prompt, false, None, None)?;

    let client = Client::new();

    let response = client
        .post(RESPONSES_URL)
        .bearer_auth(api_key.expose_secret())
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

fn extract_reasoning_summary(outputs: &[MessageOutput]) -> Option<String> {
    outputs
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
        .filter(|s| !s.is_empty())
}

fn extract_message_text(outputs: &[MessageOutput]) -> Option<String> {
    outputs
        .iter()
        .find(|o| o.output_type == "message")
        .and_then(|msg| {
            msg.content
                .iter()
                .find(|c| c.content_type == "output_text")
                .map(|c| c.text.clone())
        })
}

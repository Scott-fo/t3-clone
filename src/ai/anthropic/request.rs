use crate::models::message::Message;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AnthropicMessage<'a> {
    pub role: &'a str,
    pub content: &'a str,
}

#[derive(Debug, Serialize)]
pub struct AnthropicRequest<'a> {
    pub model: &'a str,
    pub max_tokens: u32,
    pub messages: Vec<AnthropicMessage<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl<'a> AnthropicRequest<'a> {
    pub fn chat(model: &'a str, history: &'a [Message], stream: bool) -> Self {
        Self {
            model,
            max_tokens: 1024,
            system: None,
            stream: Some(stream),
            messages: history
                .iter()
                .map(|m| AnthropicMessage {
                    role: &m.role,
                    content: &m.body,
                })
                .collect(),
        }
    }

    pub fn prompt(model: &'a str, text: &'a str) -> Self {
        Self {
            model,
            max_tokens: 32,
            system: None,
            stream: None,
            messages: vec![AnthropicMessage {
                role: "user",
                content: text,
            }],
        }
    }
}

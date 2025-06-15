use crate::models::message::Message;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OpenRouterMessage<'a> {
    pub role: &'a str,
    pub content: &'a str,
}

#[derive(Debug, Serialize)]
pub struct OpenRouterRequest<'a> {
    pub model: &'a str,
    pub messages: Vec<OpenRouterMessage<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

impl<'a> OpenRouterRequest<'a> {
    pub fn chat(model: &'a str, history: &'a [Message], stream: bool) -> Self {
        Self {
            model,
            messages: history
                .iter()
                .map(|m| OpenRouterMessage {
                    role: &m.role,
                    content: &m.body,
                })
                .collect(),
            stream: Some(stream),
            max_tokens: None,
        }
    }

    pub fn prompt(model: &'a str, text: &'a str) -> Self {
        Self {
            model,
            messages: vec![OpenRouterMessage {
                role: "user",
                content: text,
            }],
            stream: None,
            max_tokens: Some(32),
        }
    }
}

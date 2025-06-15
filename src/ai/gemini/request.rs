use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GeminiRequest<'a> {
    pub contents: Vec<GeminiMessage<'a>>,
}

#[derive(Debug, Serialize)]
pub struct GeminiMessage<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<&'a str>,
    pub parts: Vec<GeminiPart<'a>>,
}

#[derive(Debug, Serialize)]
pub struct GeminiPart<'a> {
    pub text: &'a str,
}

impl<'a> GeminiRequest<'a> {
    pub fn chat(history: &'a [crate::models::message::Message]) -> Self {
        let contents = history
            .iter()
            .map(|m| GeminiMessage {
                role: Some(match m.role.as_str() {
                    "assistant" => "model",
                    _ => "user",
                }),
                parts: vec![GeminiPart { text: &m.body }],
            })
            .collect();

        Self { contents }
    }

    pub fn prompt(text: &'a str) -> Self {
        Self {
            contents: vec![GeminiMessage {
                role: Some("user"),
                parts: vec![GeminiPart { text }],
            }],
        }
    }
}

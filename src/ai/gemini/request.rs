use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<GeminiMessage>,
}

#[derive(Debug, Serialize)]
pub struct GeminiMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    pub parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize)]
pub struct GeminiPart {
    pub text: String,
}

impl GeminiRequest {
    pub fn chat(history: &[crate::models::message::Message]) -> Self {
        let contents = history
            .iter()
            .map(|m| GeminiMessage {
                role: Some(
                    match m.role.as_str() {
                        "assistant" => "model",
                        _ => "user",
                    }
                    .into(),
                ),
                parts: vec![GeminiPart {
                    text: m.body.clone(),
                }],
            })
            .collect();

        Self { contents }
    }

    pub fn prompt(text: String) -> Self {
        Self {
            contents: vec![GeminiMessage {
                role: Some("user".into()),
                parts: vec![GeminiPart { text }],
            }],
        }
    }
}

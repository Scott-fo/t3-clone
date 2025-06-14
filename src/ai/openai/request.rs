use super::model::OpenAiModel;
use crate::ai::reasoning::{EffortLevel, Reasoning};
use serde::{Deserialize, Serialize};

pub const RESPONSES_URL: &str = "https://api.openai.com/v1/responses";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Turn {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input {
    Text(String),
    Chat(Vec<Turn>),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAiRequest {
    #[serde(rename = "model")]
    model: OpenAiModel,

    input: Input,
    stream: bool,

    instructions: Option<String>,
    reasoning: Option<Reasoning>,
}

impl OpenAiRequest {
    pub fn prompt(
        model: OpenAiModel,
        text: impl Into<String>,
        stream: bool,
        effort: Option<EffortLevel>,
        instructions: Option<String>,
    ) -> anyhow::Result<Self> {
        Self::new(
            model,
            Input::Text(text.into()),
            stream,
            effort,
            instructions,
        )
    }

    pub fn chat(
        model: OpenAiModel,
        turns: Vec<Turn>,
        effort: Option<EffortLevel>,
        instructions: Option<String>,
    ) -> anyhow::Result<Self> {
        Self::new(model, Input::Chat(turns), true, effort, instructions)
    }

    fn new(
        model: OpenAiModel,
        input: Input,
        stream: bool,
        effort: Option<EffortLevel>,
        instructions: Option<String>,
    ) -> anyhow::Result<Self> {
        let reasoning = if model.requires_reasoning() {
            Some(Reasoning::new(effort.ok_or_else(|| {
                anyhow::anyhow!("model {model} requires reasoning")
            })?))
        } else {
            None
        };

        Ok(Self {
            model,
            input,
            stream,
            instructions,
            reasoning,
        })
    }
}

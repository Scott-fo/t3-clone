use super::model::OpenAiModel;
use crate::ai::reasoning::{EffortLevel, Reasoning};
use serde::{Deserialize, Serialize};

pub const RESPONSES_URL: &str = "https://api.openai.com/v1/responses";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Turn<'a> {
    pub role: &'a str,
    pub content: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input<'a> {
    Text(&'a str),
    Chat(Vec<Turn<'a>>),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAiRequest<'a> {
    #[serde(rename = "model")]
    model: OpenAiModel,

    input: Input<'a>,
    stream: bool,

    instructions: Option<&'a str>,
    reasoning: Option<Reasoning>,
}

impl<'a> OpenAiRequest<'a> {
    pub fn prompt(
        model: OpenAiModel,
        text: &'a str,
        stream: bool,
        effort: Option<EffortLevel>,
        instructions: Option<&'a str>,
    ) -> anyhow::Result<Self> {
        Self::new(model, Input::Text(text), stream, effort, instructions)
    }

    pub fn chat(
        model: OpenAiModel,
        turns: Vec<Turn<'a>>,
        effort: Option<EffortLevel>,
        instructions: Option<&'a str>,
    ) -> anyhow::Result<Self> {
        Self::new(model, Input::Chat(turns), true, effort, instructions)
    }

    fn new(
        model: OpenAiModel,
        input: Input<'a>,
        stream: bool,
        effort: Option<EffortLevel>,
        instructions: Option<&'a str>,
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

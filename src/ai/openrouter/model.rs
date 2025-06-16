use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum OpenRouterModel {
    #[serde(rename = "google/gemini-2.5-flash-preview-05-20")]
    #[strum(serialize = "google/gemini-2.5-flash-preview-05-20")]
    GeminiFlash25,

    #[serde(rename = "google/gemini-2.5-pro-preview")]
    #[strum(serialize = "google/gemini-2.5-pro-preview")]
    Gemini25Pro,

    #[serde(rename = "openai/gpt-4.1")]
    #[strum(serialize = "openai/gpt-4.1")]
    OpenaiGpt41,
}

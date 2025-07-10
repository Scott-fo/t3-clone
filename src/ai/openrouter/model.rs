use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum OpenRouterModel {
    #[serde(rename = "google/gemini-2.5-flash")]
    #[strum(serialize = "google/gemini-2.5-flash")]
    GeminiFlash25,

    #[serde(rename = "x-ai/grok-4")]
    #[strum(serialize = "x-ai/grok-4")]
    Grok4,
}

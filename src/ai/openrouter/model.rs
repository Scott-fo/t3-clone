use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum OpenRouterModel {
    #[serde(rename = "deepseek/deepseek-r1-0528-qwen3-8b:free")]
    #[strum(serialize = "deepseek/deepseek-r1-0528-qwen3-8b:free")]
    DeepseekR1Qwen3_8BFree,

    #[serde(rename = "google/gemini-2.0-flash-exp:free")]
    #[strum(serialize = "google/gemini-2.0-flash-exp:free")]
    GeminiFlash20Free,
}

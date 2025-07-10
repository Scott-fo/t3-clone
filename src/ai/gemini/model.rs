use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum GeminiModel {
    #[serde(rename = "gemini-2.5-pro")]
    #[strum(serialize = "gemini-2.5-pro")]
    Pro25,

    #[serde(rename = "gemini-2.5-flash")]
    #[strum(serialize = "gemini-2.5-flash")]
    Flash25,

    #[serde(rename = "gemini-2.0-flash")]
    #[strum(serialize = "gemini-2.0-flash")]
    Flash20,
}

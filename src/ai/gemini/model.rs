use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum GeminiModel {
    #[serde(rename = "gemini-2.5-pro-preview-06-05")]
    #[strum(serialize = "gemini-2.5-pro-preview-06-05")]
    Pro25,

    #[serde(rename = "gemini-2.5-flash-preview-05-20")]
    #[strum(serialize = "gemini-2.5-flash-preview-05-20")]
    Flash25,

    #[serde(rename = "gemini-2.0-flash")]
    #[strum(serialize = "gemini-2.0-flash")]
    Flash20,
}

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumString, Display, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum OpenAiModel {
    #[serde(rename = "gpt-4o")]
    #[strum(serialize = "gpt-4o")]
    Gpt4o,

    #[serde(rename = "gpt-4.1")]
    #[strum(serialize = "gpt-4.1")]
    Gpt41,

    #[serde(rename = "gpt-4.1-mini")]
    #[strum(serialize = "gpt-4.1-mini")]
    Gpt41Mini,

    #[serde(rename = "gpt-4.1-nano")]
    #[strum(serialize = "gpt-4.1-nano")]
    Gpt41Nano,

    #[serde(rename = "o3-mini")]
    #[strum(serialize = "o3-mini")]
    O3Mini,

    #[serde(rename = "o4-mini")]
    #[strum(serialize = "o4-mini")]
    O4Mini,

    #[serde(rename = "o3")]
    #[strum(serialize = "o3")]
    O3,
}

impl OpenAiModel {
    #[inline]
    pub fn requires_reasoning(self) -> bool {
        matches!(self, Self::O3 | Self::O3Mini | Self::O4Mini)
    }
}

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum AnthropicModel {
    #[serde(rename = "claude-3-5-haiku-latest")]
    #[strum(serialize = "claude-3-5-haiku-latest")]
    Haiku35,

    #[serde(rename = "claude-sonnet-4-20250514")]
    #[strum(serialize = "claude-sonnet-4-20250514")]
    Sonnet4,

    #[serde(rename = "claude-opus-4-20250514")]
    #[strum(serialize = "claude-opus-4-20250514")]
    Opus4,
}

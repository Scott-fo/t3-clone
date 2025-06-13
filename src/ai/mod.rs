pub mod google;
pub mod handler;
pub mod openai;
pub mod reasoning;

use std::str::FromStr;

use anyhow::{Error, bail};

#[derive(Debug, Clone, PartialEq)]
pub enum AiProvider {
    OpenAi,
    Google,
}

impl FromStr for AiProvider {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "openai" => Ok(Self::OpenAi),
            "google" => Ok(Self::Google),
            _ => bail!("Invalid AI provider: '{}'", s),
        }
    }
}

impl std::fmt::Display for AiProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiProvider::OpenAi => write!(f, "openai"),
            AiProvider::Google => write!(f, "google"),
        }
    }
}

use std::str::FromStr;

use anyhow::{Context, Error, Result, bail};
use diesel::MysqlConnection;
use secrecy::SecretString;
use thiserror::Error;

use crate::app::AppState;

use super::reasoning::EffortLevel;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("API key missing for provider {0}")]
    MissingApiKey(AiProvider),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ProviderResult<T> = std::result::Result<T, ProviderError>;

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

#[derive(Debug)]
pub struct ProviderSetup {
    pub provider: AiProvider,
    pub model: String,
    pub effort: Option<EffortLevel>,
    pub api_key: SecretString,
}

pub fn pick_provider(
    state: &AppState,
    conn: &mut MysqlConnection,
    user_id: &str,
) -> ProviderResult<ProviderSetup> {
    let mut model = "gpt-4.1-mini".to_owned();
    let mut provider = AiProvider::OpenAi;
    let mut effort: Option<EffortLevel> = None;

    if let Some(active) = state
        .service_container
        .active_model_service
        .get_for_user(conn, user_id)
        .context("query active_model")
        .map_err(ProviderError::Other)?
    {
        provider = active.provider.parse()?;
        model = active.model;
        effort = active.reasoning.as_deref().and_then(|s| s.parse().ok());
    }

    let api_key = state
        .service_container
        .api_key_service
        .get_and_decrypt(conn, user_id, &provider.to_string())
        .map_err(|_| ProviderError::MissingApiKey(provider.clone()))?;

    Ok(ProviderSetup {
        provider,
        model,
        effort,
        api_key,
    })
}

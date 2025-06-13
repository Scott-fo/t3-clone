use anyhow::{Context, Result};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use secrecy::SecretString;
use serde::Deserialize;

use crate::{app::AppState, dtos, models::api_key::CreateArgs};

#[derive(Debug, Deserialize)]
pub struct ApiKeyCreateRequest {
    pub provider: String,
    #[serde(rename = "key")]
    pub api_key: SecretString,
}

#[tracing::instrument(
    skip(state, user, payload),
    fields(user_id = %user.id, provider = %payload.provider)
)]
pub async fn create_api_key(
    State(state): State<AppState>,
    Extension(user): Extension<dtos::user::User>,
    Json(payload): Json<ApiKeyCreateRequest>,
) -> Result<(StatusCode, Json<dtos::api_key::ApiKey>), (StatusCode, String)> {
    let mut conn = state
        .db_pool
        .get()
        .context("DB pool")
        .map_err(internal_error)?;

    let args = CreateArgs {
        provider: payload.provider,
        api_key: payload.api_key,
    };
    let created = state
        .service_container
        .api_key_service
        .create(&mut conn, &user.id, args)
        .context("service")
        .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(created.into())))
}

#[tracing::instrument(skip(state, user))]
pub async fn list_api_keys(
    State(state): State<AppState>,
    Extension(user): Extension<dtos::user::User>,
) -> Result<Json<Vec<dtos::api_key::ApiKey>>, (StatusCode, String)> {
    let mut conn = state
        .db_pool
        .get()
        .context("DB pool")
        .map_err(internal_error)?;

    let list = state
        .service_container
        .api_key_service
        .list(&mut conn, &user.id)
        .context("service")
        .map_err(internal_error)?;

    Ok(Json(
        list.into_iter().map(dtos::api_key::ApiKey::from).collect(),
    ))
}

#[tracing::instrument(skip(state, user))]
pub async fn delete_api_key(
    State(state): State<AppState>,
    Extension(user): Extension<dtos::user::User>,
    Path(id): Path<u64>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = state
        .db_pool
        .get()
        .context("DB pool")
        .map_err(internal_error)?;

    state
        .service_container
        .api_key_service
        .delete(&mut conn, id, &user.id)
        .context("service")
        .map_err(internal_error)?;

    Ok(StatusCode::NO_CONTENT)
}

fn internal_error<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    tracing::error!("{e}");
    (StatusCode::INTERNAL_SERVER_ERROR, "internal error".into())
}

use anyhow::{Context, Result};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::instrument;

use crate::{app::AppState, dtos};

// Public
#[instrument(skip(state))]
pub async fn get_shared_chat(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<dtos::shared_chat::SharedChatWithMessages>, (StatusCode, String)> {
    let mut conn = state
        .db_pool
        .get()
        .context("DB pool")
        .map_err(internal_error)?;

    let snapshot = state
        .service_container
        .shared_chat_service
        .get(&mut conn, &id)
        .context("service")
        .map_err(internal_error)?;

    Ok(Json(snapshot.into()))
}

#[instrument(skip(state, user), fields(user_id=%user.id, chat_id=%chat_id))]
pub async fn create_shared_chat(
    State(state): State<AppState>,
    Extension(user): Extension<dtos::user::User>,
    Path(chat_id): Path<String>,
) -> Result<(StatusCode, Json<dtos::shared_chat::SharedChatWithMessages>), (StatusCode, String)> {
    let mut conn = state
        .db_pool
        .get()
        .context("DB pool")
        .map_err(internal_error)?;

    let snapshot = state
        .service_container
        .shared_chat_service
        .create(&mut conn, &chat_id, &user.id)
        .context("service")
        .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(snapshot.into())))
}

#[instrument(skip(state, user), fields(user_id=%user.id, shared_chat_id=%id))]
pub async fn delete_shared_chat(
    State(state): State<AppState>,
    Extension(user): Extension<dtos::user::User>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = state
        .db_pool
        .get()
        .context("DB pool")
        .map_err(internal_error)?;

    state
        .service_container
        .shared_chat_service
        .delete(&mut conn, &id, &user.id)
        .context("service")
        .map_err(internal_error)?;

    Ok(StatusCode::NO_CONTENT)
}

fn internal_error<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    tracing::error!("{e}");
    (StatusCode::INTERNAL_SERVER_ERROR, "internal error".into())
}

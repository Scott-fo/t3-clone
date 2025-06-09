use anyhow::{Context, Result};
use axum::{Extension, Json, extract::State, http::StatusCode};
use secrecy::SecretString;
use serde::Deserialize;
use tower_sessions::Session;

use crate::{
    app::AppState,
    dtos,
    models::session::Session as DbSession,
    models::user::{NewUser, User},
    repositories::{Repository, session::SessionRepository},
};

#[tracing::instrument(skip(user))]
pub async fn get_current_user(
    Extension(user): Extension<dtos::user::User>,
) -> Json<dtos::user::User> {
    Json(user)
}

#[derive(Debug, Deserialize)]
pub struct LogInRequest {
    pub email: String,
    pub password: SecretString,
}

#[axum::debug_handler]
#[tracing::instrument(skip(state, session))]
pub async fn login(
    State(state): State<AppState>,
    session: Session,
    Json(credentials): Json<LogInRequest>,
) -> Result<Json<dtos::user::User>, (StatusCode, String)> {
    let (user, db_session) = User::authenticate_and_create_session(
        &credentials.email,
        &credentials.password,
        &state.db_pool,
    )
    .map_err(|e| {
        tracing::error!("Authentication failed: {:?}", e);

        if e.to_string() == "Invalid email or password" {
            (StatusCode::UNAUTHORIZED, e.to_string())
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal error occurred".to_string(),
            )
        }
    })?;

    session
        .insert("session_id", &db_session.id)
        .await
        .context("Failed to store session id in user's cookie")
        .map_err(|e| {
            tracing::error!("Session storage failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to process session".to_string(),
            )
        })?;

    Ok(Json(dtos::user::User::from(user)))
}

#[tracing::instrument(
    skip(state, session),
    fields(session_id=tracing::field::Empty)
)]
pub async fn logout(
    State(state): State<AppState>,
    session: Session,
) -> Result<StatusCode, (StatusCode, String)> {
    let session_id_option = session
        .get::<String>("session_id")
        .await
        .context("Failed to access session store")
        .map_err(|e| {
            tracing::error!("Session store error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error accessing session".to_string(),
            )
        })?;

    if let Some(session_id) = session_id_option {
        tracing::Span::current().record("session_id", &tracing::field::display(&session_id));

        let mut conn = state
            .db_pool
            .get()
            .context("Failed to get DB connection")
            .map_err(|e| {
                tracing::error!("DB connection pool error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database is unavailable".to_string(),
                )
            })?;

        SessionRepository
            .delete(&mut conn, &session_id)
            .context("Failed to delete session from database")
            .map_err(|e| {
                tracing::error!("DB session deletion error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Could not process sign-out".to_string(),
                )
            })?;

        session
            .delete()
            .await
            .context("Failed to delete cookie session")
            .map_err(|e| {
                tracing::error!("Cookie session deletion error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Could not process sign-out".to_string(),
                )
            })?;

        tracing::info!("User signed out successfully.");
    } else {
        tracing::info!("Sign-out attempted for a user with no active session.");
    }

    Ok(StatusCode::OK)
}

#[tracing::instrument(
    skip(state, payload),
    fields(user_email = %payload.email)
)]
pub async fn register(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<NewUser>,
) -> Result<(StatusCode, Json<dtos::user::User>), (StatusCode, String)> {
    let mut conn = state
        .db_pool
        .get()
        .context("Failed to get DB connection from pool")
        .map_err(|e| {
            tracing::error!("DB connection pool error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "The server is currently unavailable".to_string(),
            )
        })?;

    // String error handling for now. define a concrete error type later to use to make this better
    let new_user = User::create(payload, &mut conn).map_err(|e| {
        tracing::warn!("User registration failed: {:?}", e);

        let error_string = e.to_string();

        let status = match error_string.as_str() {
            "Password should be at least 12 characters" => StatusCode::BAD_REQUEST,
            "Passwords do not match" => StatusCode::BAD_REQUEST,
            "Email already exists" => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = if status.is_server_error() {
            "An internal error occurred during registration".to_string()
        } else {
            error_string
        };

        (status, message)
    })?;

    tracing::info!("New user registered successfully: {}", new_user.email);
    let new_session = DbSession::new(&new_user.id);

    let db_session = SessionRepository
        .create(&mut conn, &new_session)
        .context("Failed to create database session")
        .map_err(|e| {
            tracing::error!("Database session creation failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to process your session".to_string(),
            )
        })?;

    session
        .insert("session_id", &db_session.id)
        .await
        .context("Failed to store session id in user's cookie")
        .map_err(|e| {
            tracing::error!("Cookie session insertion failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to process your session".to_string(),
            )
        })?;

    Ok((StatusCode::CREATED, Json(dtos::user::User::from(new_user))))
}

use axum::{extract::State, http::StatusCode, middleware::Next, response::Response};
use diesel::RunQueryDsl;
use diesel::query_dsl::methods::FindDsl;
use tower_sessions::Session;

use crate::app::AppState;
use crate::dtos;
use crate::models::user::User;
use crate::repositories::Repository;
use crate::repositories::session::SessionRepository;

#[tracing::instrument(skip_all)]
pub async fn auth(
    State(state): State<AppState>,
    session: Session,
    mut request: axum::http::Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    use crate::schema::users;

    let session_id = match session.get::<String>("session_id").await {
        Err(e) => {
            tracing::error!("Failed to access session store: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Ok(Some(id)) => id,
    };

    let mut conn = state.db_pool.get().map_err(|e| {
        tracing::error!("DB connection pool error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let db_session = match SessionRepository.find_by_id(&mut conn, &session_id) {
        Err(e) => {
            tracing::error!("DB error finding session: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Ok(None) => {
            tracing::warn!("Invalid session ID presented: {}", session_id);
            return Err(StatusCode::UNAUTHORIZED);
        }
        Ok(Some(session)) => session,
    };

    // TODO: Expired session check

    let user = match users::table
        .find(&db_session.user_id)
        .first::<User>(&mut conn)
    {
        Err(diesel::result::Error::NotFound) => {
            tracing::error!(
                "Session {} references non-existent user {}",
                db_session.id,
                db_session.user_id
            );
            return Err(StatusCode::UNAUTHORIZED);
        }
        Err(e) => {
            tracing::error!("DB error finding user for session: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Ok(user) => user,
    };

    request
        .extensions_mut()
        .insert(dtos::user::User::from(user));

    Ok(next.run(request).await)
}

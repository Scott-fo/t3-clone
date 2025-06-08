use anyhow::{Context, Result};
use axum::{Extension, Json, extract::State, http::StatusCode};

use crate::{
    app::AppState,
    dtos,
    services::replicache::{
        pull::{
            build_response, map_anyhow_error_to_response, process_pull_request, retrieve_base_cvr,
        },
        push::process_mutations,
        types::{PullRequest, PullResponse, PushRequest, PushResponse},
    },
};

#[tracing::instrument(
    skip(state, user),
    fields(user_id = tracing::field::Empty, client_group_id = tracing::field::Empty)
)]
pub async fn replicache_pull(
    State(state): State<AppState>,
    Extension(user): Extension<dtos::user::User>,
    Json(pull_req): Json<PullRequest>,
) -> Result<Json<PullResponse>, (StatusCode, String)> {
    let result = async {
        let base_cvr = retrieve_base_cvr(&pull_req.cookie, state.cache.clone()).await?;
        let cookie = pull_req.cookie.clone();

        let pull_result = tokio::task::spawn_blocking(move || {
            process_pull_request(
                &state.db_pool,
                base_cvr,
                &user.id,
                &pull_req.client_group_id.clone(),
                cookie.map(|c| c.order).unwrap_or(0),
                &state.service_container.clone(),
            )
        })
        .await
        .context("Task panicked or was cancelled")??;

        build_response(pull_result, pull_req.cookie, state.cache).await
    }
    .await;

    result.map_err(map_anyhow_error_to_response)
}

#[tracing::instrument(
    skip(state, user, push_req),
    fields(
        client_group_id = %push_req.client_group_id,
        user_id = %user.id,
        mutation_count = push_req.mutations.len(),
    ),
)]
pub async fn replicache_push(
    State(state): State<AppState>,
    Extension(user): Extension<dtos::user::User>,
    Json(push_req): Json<PushRequest>,
) -> Result<Json<PushResponse>, (StatusCode, String)> {
    let result = async {
        let user_id = user.id.clone();
        let s = state.clone();

        tokio::task::spawn_blocking(move || {
            process_mutations(s, &push_req.client_group_id, &user_id, &push_req.mutations)
        })
        .await
        .context("Task panicked or was cancelled")??;

        state.sse_manager.replicache_poke(&user.id).await;
        tracing::info!("Broadcasted poke to user {}", user.id);

        Ok(Json(PushResponse { success: true }))
    }
    .await;

    result.map_err(map_anyhow_error_to_response)
}

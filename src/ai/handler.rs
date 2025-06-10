use anyhow::{Context, Result};
use chrono::Utc;
use tokio_retry2::{
    Retry, RetryError,
    strategy::{ExponentialBackoff, jitter},
};

use crate::{
    app::AppState,
    models::message::{CreateArgs, Message},
};

pub fn spawn_title_generation_task(
    state: AppState,
    chat_id: String,
    message_body: String,
    user_id: String,
) {
    tokio::spawn(async move {
        let strategy = ExponentialBackoff::from_millis(1000).map(jitter).take(3);

        let action = || async {
            generate_and_save_title(&state, &chat_id, &message_body, &user_id)
                .await
                .map_err(RetryError::transient)
        };

        match Retry::spawn(strategy, action).await {
            Ok(_) => {
                tracing::info!(
                    "Successfully generated and saved title for chat {}",
                    chat_id
                );
            }
            Err(e) => {
                tracing::error!(
                    error = ?e,
                    "Failed to generate and save title for chat {} after multiple retries.",
                    chat_id
                );
            }
        }
    });
}

async fn generate_and_save_title(
    state: &AppState,
    chat_id: &str,
    message_body: &str,
    user_id: &str,
) -> Result<()> {
    let title = super::openai::generate_title(&message_body)
        .await
        .context("OpenAI title generation failed")?;

    let mut conn = state
        .db_pool
        .get()
        .context("Failed to get DB connection from pool")?;

    let _chat_to_update = state
        .service_container
        .chat_service
        .get(&mut conn, &chat_id, &user_id)
        .context(format!(
            "Failed to find chat {} to update its title",
            chat_id
        ))?;

    let update_args = crate::models::chat::UpdateArgs {
        id: chat_id.to_string(),
        title: Some(title),
        pinned: None,
        archived: None,
        updated_at: Utc::now(),
    };

    state
        .service_container
        .chat_service
        .update(&mut conn, update_args, &user_id)
        .context(format!("Failed to update chat {} with new title", chat_id))?;

    state.sse_manager.replicache_poke(&user_id).await;

    Ok(())
}

pub fn spawn_chat_task(state: AppState, user_id: String, args: CreateArgs) {
    let sse_manager = state.sse_manager;

    let mut conn = match state.db_pool.get() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(error = %e, "Failed to get DB connection from pool. Aborting save.");
            return;
        }
    };

    let mut model = "gpt-4.1-mini".to_string();
    let mut provider = super::AiProvider::OpenAi;

    if let Ok(Some(active)) = state
        .service_container
        .active_model_service
        .get_for_user(&mut conn, &user_id)
    {
        if let Ok(p) = active.provider.parse() {
            provider = p;
        }

        model = active.model;
    } else {
        tracing::warn!(user = %user_id, "Failed to find active model for user");
    }

    tracing::info!(model = %model, provider = %provider, "Using model and provider");

    let messages: Vec<Message> = match state.service_container.message_service.list_for_chat(
        &mut conn,
        &args.chat_id,
        &user_id,
    ) {
        Ok(msgs) => msgs,
        Err(e) => {
            tracing::error!(error = %e, "Failed to list messages for chat. Aborting AI response.");
            return;
        }
    };

    let previous_response_id = messages
        .iter()
        .rev()
        .find(|m| m.role == "assistant")
        .map(|m| m.id.clone());

    tokio::spawn(async move {
        let result = match provider {
            super::AiProvider::OpenAi => {
                super::openai::stream_openai_response(
                    sse_manager,
                    user_id.clone(),
                    args.chat_id.clone(),
                    args.body,
                    model,
                    previous_response_id,
                )
                .await
            }
        };

        match result {
            Ok(Some((msg_id, final_content))) => {
                tracing::info!(%msg_id, %final_content, "Stream completed. Saving to DB or processing...");

                let ca = CreateArgs {
                    id: msg_id,
                    chat_id: args.chat_id,
                    role: "assistant".to_string(),
                    body: final_content,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };

                if let Err(e) = state
                    .service_container
                    .message_service
                    .create(&mut conn, ca, &user_id)
                {
                    tracing::error!(error = %e, "Failed to save assistant message to DB.");
                }
            }
            Ok(None) => {
                tracing::warn!("Stream finished but did not return a message.");
            }
            Err(e) => {
                tracing::error!(error = %e, "The OpenAI stream task failed.");
            }
        }
    });
}

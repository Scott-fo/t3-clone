use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use serde_json::json;
use tokio_retry2::{
    Retry, RetryError,
    strategy::{ExponentialBackoff, jitter},
};

use crate::{
    ai::reasoning::EffortLevel,
    app::AppState,
    models::message::{CreateArgs, Message},
    services::sse_manager::{EventType, SseMessage},
};

// MAJOR REFACTOR NEEDED

pub struct StreamResult {
    pub msg_id: String,
    pub content: String,
    pub reasoning: Option<String>,
}

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
    let mut conn = state
        .db_pool
        .get()
        .context("Failed to get DB connection from pool")?;

    let api_key = state
        .service_container
        .api_key_service
        .get_for_provider(&mut conn, &user_id, &super::AiProvider::OpenAi.to_string())
        .and_then(|key| state.service_container.api_key_service.decrypt(key))?;

    let title = super::openai::generate_title(&api_key, &message_body)
        .await
        .context("OpenAI title generation failed")?;

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
        pinned_at: None,
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

pub fn spawn_chat_task(state: AppState, user_id: String, args: CreateArgs, messages: Vec<Message>) {
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
    let mut effort: Option<EffortLevel> = None;

    if let Ok(Some(active)) = state
        .service_container
        .active_model_service
        .get_for_user(&mut conn, &user_id)
    {
        if let Ok(p) = active.provider.parse() {
            provider = p;
        }

        effort = active.reasoning.as_deref().and_then(|s| s.parse().ok());

        model = active.model;
    } else {
        tracing::warn!(user = %user_id, "Failed to find active model for user");
    }

    tracing::info!(model = %model, provider = %provider, "Using model and provider");

    let api_key = match state
        .service_container
        .api_key_service
        .get_for_provider(&mut conn, &user_id, &provider.to_string())
        .and_then(|key| state.service_container.api_key_service.decrypt(key))
    {
        Ok(k) => k,
        Err(_) => {
            tracing::error!(%user_id, "API key missing for provider");

            // to sort ordering issue in dev.
            let now = Utc::now();
            let safe_created = if now.timestamp() == args.created_at.timestamp() {
                args.created_at + Duration::seconds(1)
            } else {
                now
            };

            let ca = CreateArgs {
                id: uuid::Uuid::new_v4().to_string(),
                chat_id: args.chat_id.clone(),
                role: "assistant".to_string(),
                body: format!("Error: Missing API key for {}", provider.to_string()),
                reasoning: None,
                created_at: safe_created,
                updated_at: safe_created,
            };

            if let Err(e) = state
                .service_container
                .message_service
                .create(&mut conn, ca, &user_id)
            {
                tracing::error!(error = %e, "Failed to save assistant message to DB.");
            }

            {
                let sse_manager = sse_manager.clone();
                let user_id = user_id.clone();
                tokio::spawn(async move {
                    let exit_payload = json!({
                        "chat_id": args.chat_id,
                    });

                    sse_manager
                        .send_to_user(
                            &user_id,
                            SseMessage {
                                event_type: EventType::Exit,
                                data: Some(exit_payload),
                            },
                        )
                        .await;
                    sse_manager.replicache_poke(&user_id).await;
                });
            }
            return;
        }
    };

    tokio::spawn(async move {
        let result = match provider {
            super::AiProvider::OpenAi => {
                super::openai::stream_openai_response(
                    api_key,
                    sse_manager,
                    user_id.clone(),
                    args.chat_id.clone(),
                    model,
                    effort,
                    messages,
                )
                .await
            }
            super::AiProvider::Google => {
                super::google::stream_gemini_response(
                    api_key,
                    sse_manager,
                    user_id.clone(),
                    args.chat_id.clone(),
                    model,
                    messages,
                )
                .await
            }
        };

        match result {
            Ok(Some(stream_result)) => {
                let msg_id = stream_result.msg_id;
                let content = stream_result.content;
                let reasoning = stream_result.reasoning;

                tracing::info!(%msg_id, %content, "Stream completed. Saving to DB or processing...");

                let ca = CreateArgs {
                    id: msg_id,
                    chat_id: args.chat_id,
                    role: "assistant".to_string(),
                    body: content,
                    reasoning,
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
                tracing::error!(error = %e, "The stream task failed.");
            }
        }
    });
}

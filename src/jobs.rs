use anyhow::Result;
use serde_json::json;
use tokio::sync::mpsc;
use tokio_retry2::{
    Retry, RetryError,
    strategy::{ExponentialBackoff, jitter},
};

use crate::{
    ai::provider::{
        AiProvider, GOOGLE_TITLE_MODEL, OPENAI_TITLE_MODEL, ProviderError, pick_provider,
    },
    app::AppState,
    models::message::Message,
    services::sse_manager::{EventType, SseMessage},
};

#[derive(Debug, Clone)]
pub enum Job {
    GenerateTitle {
        chat_id: String,
        user_id: String,
        first_body: String,
    },
    GenerateResponse {
        chat_id: String,
        user_id: String,
        messages: Vec<Message>,
    },
}

pub async fn run_worker(state: AppState, mut rx: mpsc::UnboundedReceiver<Job>) {
    while let Some(job) = rx.recv().await {
        let state_cloned = state.clone();

        tokio::spawn(async move {
            let strategy = ExponentialBackoff::from_millis(500).map(jitter).take(3);

            let result = Retry::spawn(strategy, || {
                let state_inner = state_cloned.clone();
                let job_inner = job.clone();
                async move {
                    handle_job(&state_inner, job_inner)
                        .await
                        .map_err(RetryError::transient)
                }
            })
            .await;

            if let Err(e) = result {
                tracing::error!(error = ?e, job = ?job, "Job permanently failed");
            }
        });
    }
}

async fn handle_job(state: &AppState, job: Job) -> Result<()> {
    match job {
        Job::GenerateTitle {
            chat_id,
            user_id,
            first_body,
        } => {
            let mut conn = state.db_pool.get()?;
            let setup = match pick_provider(state, &mut conn, &user_id) {
                Ok(s) => s,
                Err(ProviderError::MissingApiKey(p)) => {
                    state
                        .service_container
                        .message_service
                        .save_assistant_error(&mut conn, &chat_id, p.clone(), &user_id)?;

                    state
                        .sse_manager
                        .send_to_user(
                            &user_id,
                            SseMessage {
                                event_type: EventType::Exit,
                                data: Some(json!({ "chat_id": chat_id })),
                            },
                        )
                        .await;
                    state.sse_manager.replicache_poke(&user_id).await;
                    return Ok(());
                }
                Err(e) => return Err(e.into()),
            };

            let forced_model = match setup.provider {
                AiProvider::OpenAi => OPENAI_TITLE_MODEL,
                AiProvider::Google => GOOGLE_TITLE_MODEL,
            };

            let new_title = match setup.provider {
                AiProvider::OpenAi => {
                    crate::ai::openai::generate_title(&setup.api_key, &first_body, forced_model)
                        .await?
                }
                AiProvider::Google => {
                    crate::ai::google::generate_title(&setup.api_key, &first_body, forced_model)
                        .await?
                }
            };

            state
                .service_container
                .chat_service
                .update_title(&mut conn, &chat_id, &new_title, &user_id)?;

            state.sse_manager.replicache_poke(&user_id).await;
        }

        Job::GenerateResponse {
            chat_id,
            user_id,
            messages,
        } => {
            let mut conn = state.db_pool.get()?;
            let setup = match pick_provider(state, &mut conn, &user_id) {
                Ok(s) => s,
                Err(ProviderError::MissingApiKey(p)) => {
                    state
                        .service_container
                        .message_service
                        .save_assistant_error(&mut conn, &chat_id, p.clone(), &user_id)?;

                    state
                        .sse_manager
                        .send_to_user(
                            &user_id,
                            SseMessage {
                                event_type: EventType::Exit,
                                data: Some(json!({ "chat_id": chat_id})),
                            },
                        )
                        .await;
                    state.sse_manager.replicache_poke(&user_id).await;
                    return Ok(());
                }
                Err(e) => return Err(e.into()),
            };

            let stream_res = match setup.provider {
                AiProvider::OpenAi => {
                    crate::ai::openai::stream_openai_response(
                        setup.api_key,
                        state.sse_manager.clone(),
                        user_id.clone(),
                        chat_id.clone(),
                        setup.model,
                        setup.effort,
                        messages.clone(),
                    )
                    .await?
                }
                AiProvider::Google => {
                    crate::ai::google::stream_gemini_response(
                        setup.api_key,
                        state.sse_manager.clone(),
                        user_id.clone(),
                        chat_id.clone(),
                        setup.model,
                        messages.clone(),
                    )
                    .await?
                }
            };

            if let Some(stream_res) = stream_res {
                state
                    .service_container
                    .message_service
                    .save_assistant_reply(&mut conn, &chat_id, stream_res, &user_id)?;
            }
        }
    }
    Ok(())
}

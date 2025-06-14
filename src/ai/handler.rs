use anyhow::Result;
use serde_json::json;

use crate::{
    ai::provider::{AiProvider, ProviderError, pick_provider},
    app::AppState,
    jobs::Job,
    models::message::Message,
    services::sse_manager::{EventType, SseManager, SseMessage},
};

use super::{
    gemini::{self, model::GeminiModel},
    openai::{self, model::OpenAiModel},
};

pub struct StreamResult {
    pub msg_id: String,
    pub content: String,
    pub reasoning: Option<String>,
}

pub fn create_title_prompt(msg: &str) -> String {
    format!(
        "Summarize the following message into a short, concise title of 5 words or less, without quotation marks: \"{}\"",
        msg
    )
}

pub fn enqueue_ai_jobs(
    state: &AppState,
    user_id: String,
    chat_id: String,
    new_msg_body: String,
    messages: Vec<Message>,
) -> Result<()> {
    if messages.len() == 1 {
        state.job_tx.send(Job::GenerateTitle {
            chat_id: chat_id.clone(),
            user_id: user_id.clone(),
            first_body: new_msg_body,
        })?;
    }

    state.job_tx.send(Job::GenerateResponse {
        chat_id,
        user_id,
        messages: messages,
    })?;

    Ok(())
}

pub async fn generate_title(
    state: &AppState,
    chat_id: String,
    user_id: String,
    first_body: String,
) -> Result<()> {
    let (provider, api_key) = {
        let mut conn = state.db_pool.get()?;
        let setup = pick_provider(state, &mut conn, &user_id)?;

        (setup.provider, setup.api_key)
    };

    let new_title = match provider {
        AiProvider::OpenAi => {
            openai::handler::generate_title(&api_key, &first_body, OpenAiModel::Gpt41Nano).await?
        }
        AiProvider::Google => {
            gemini::handler::generate_title(&api_key, &first_body, GeminiModel::Flash20).await?
        }
    };

    {
        let mut conn = state.db_pool.get()?;
        state
            .service_container
            .chat_service
            .update_title(&mut conn, &chat_id, &new_title, &user_id)?;
    }

    state.sse_manager.replicache_poke(&user_id).await;

    Ok(())
}

pub async fn generate_response(
    state: &AppState,
    chat_id: String,
    user_id: String,
    messages: Vec<Message>,
) -> Result<()> {
    let setup = {
        let mut conn = state.db_pool.get()?;
        match pick_provider(state, &mut conn, &user_id) {
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
        }
    };

    let stream_res = match setup.provider {
        AiProvider::OpenAi => {
            openai::handler::stream(
                setup.api_key,
                state.sse_manager.clone(),
                user_id.clone(),
                chat_id.clone(),
                setup.model.parse()?,
                setup.effort,
                messages.clone(),
            )
            .await?
        }
        AiProvider::Google => {
            gemini::handler::stream(
                setup.api_key,
                state.sse_manager.clone(),
                user_id.clone(),
                chat_id.clone(),
                setup.model.parse()?,
                messages.clone(),
            )
            .await?
        }
    };

    if let Some(stream_res) = stream_res {
        let mut conn = state.db_pool.get()?;
        state
            .service_container
            .message_service
            .save_assistant_reply(&mut conn, &chat_id, stream_res, &user_id)?;
    }

    Ok(())
}

pub async fn send_text_delta(sse: &SseManager, user: &str, chat: &str, delta: &str) {
    let payload = json!({ "chat_id": chat, "chunk": delta });
    sse.send_to_user(
        user,
        SseMessage {
            event_type: EventType::Chunk,
            data: Some(payload),
        },
    )
    .await;
}

pub async fn send_reasoning_delta(sse: &SseManager, user: &str, chat: &str, delta: &str) {
    let payload = json!({ "chat_id": chat, "reasoning": delta });
    sse.send_to_user(
        user,
        SseMessage {
            event_type: EventType::Chunk,
            data: Some(payload),
        },
    )
    .await;
}

pub async fn send_error(sse: &SseManager, user: &str, chat: &str, error: &str) {
    let payload = json!({ "chat_id": chat, "error": error });
    sse.send_to_user(
        user,
        SseMessage {
            event_type: EventType::Err,
            data: Some(payload),
        },
    )
    .await;
}

pub async fn done(sse: &SseManager, user: &str, chat: &str, id: &str) {
    let payload = json!({ "chat_id": chat, "msg_id": id });
    sse.send_to_user(
        user,
        SseMessage {
            event_type: EventType::Done,
            data: Some(payload),
        },
    )
    .await;
}

use anyhow::Result;
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;

use crate::ai;
use crate::app::AppState;
use crate::models::message::{CreateArgs, DeleteArgs, Message, UpdateArgs};

use super::handler::Mutation;

#[derive(Debug, Deserialize)]
#[serde(tag = "name", content = "args")]
pub enum MessageMutation {
    #[serde(rename = "createMessage")]
    Create(CreateArgs),
    #[serde(rename = "updateMessage")]
    Update(UpdateArgs),
    #[serde(rename = "deleteMessage")]
    Delete(DeleteArgs),
}

impl MessageMutation {}

impl Mutation for MessageMutation {
    fn process(
        &self,
        state: AppState,
        conn: &mut MysqlConnection,
        user_id: &str,
    ) -> Result<Option<String>> {
        match self {
            MessageMutation::Create(args) => {
                let msg =
                    state
                        .service_container
                        .message_service
                        .create(conn, args.clone(), user_id)?;

                if args.role == "user" {
                    let messages = state.service_container.message_service.list_for_chat(
                        conn,
                        &args.chat_id,
                        user_id,
                    )?;

                    if messages.len() == 1 {
                        tracing::info!(
                            "First message detected in chat {}. Spawning title generation task.",
                            args.chat_id
                        );
                        spawn_title_generation_task(
                            state.clone(),
                            args.chat_id.clone(),
                            args.body.clone(),
                            user_id.to_string(),
                        );
                    }
                    start_chat_handler(state, user_id.to_string(), args.to_owned());
                }

                Ok(Some(msg.id))
            }
            MessageMutation::Update(args) => {
                let msg =
                    state
                        .service_container
                        .message_service
                        .update(conn, args.clone(), user_id)?;
                Ok(Some(msg.id))
            }
            MessageMutation::Delete(args) => {
                let msg = state
                    .service_container
                    .message_service
                    .delete(conn, &args.id, user_id)?;
                Ok(Some(msg.id))
            }
        }
    }
}

fn spawn_title_generation_task(
    state: AppState,
    chat_id: String,
    message_body: String,
    user_id: String,
) {
    tokio::spawn(async move {
        match ai::openai::generate_title(&message_body).await {
            Ok(title) => {
                tracing::info!(
                    "Successfully generated title '{}' for chat {}",
                    title,
                    chat_id
                );

                let mut conn = match state.db_pool.get() {
                    Ok(c) => c,
                    Err(e) => {
                        tracing::error!(error = %e, "Failed to get DB connection from pool for title update.");
                        return;
                    }
                };

                let _chat_to_update = match state
                    .service_container
                    .chat_service
                    .get(&mut conn, &chat_id, &user_id)
                {
                    Ok(c) => c,
                    _ => {
                        tracing::error!("Failed to find chat {} to update its title.", chat_id);
                        return;
                    }
                };

                let update_args = crate::models::chat::UpdateArgs {
                    id: chat_id.clone(),
                    title: Some(title),
                    pinned: None,
                    archived: None,
                    updated_at: Utc::now(),
                };

                if let Err(e) =
                    state
                        .service_container
                        .chat_service
                        .update(&mut conn, update_args, &user_id)
                {
                    tracing::error!(error = %e, "Failed to update chat {} with new title.", chat_id);
                }
            }
            Err(e) => {
                tracing::error!(error = %e, "Failed to generate title for chat {}", chat_id);
            }
        }
    });
}

fn start_chat_handler(state: AppState, user_id: String, args: CreateArgs) {
    let model = "gpt-4.1".to_string();

    let sse_manager = state.sse_manager;

    let mut conn = match state.db_pool.get() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(error = %e, "Failed to get DB connection from pool. Aborting save.");
            return;
        }
    };

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
        let result = ai::openai::stream_openai_response(
            sse_manager,
            user_id.clone(),
            args.chat_id.clone(),
            args.body,
            model,
            previous_response_id,
        )
        .await;

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

use anyhow::Result;
use diesel::prelude::*;
use serde::Deserialize;

use crate::ai;
use crate::app::AppState;
use crate::models::message::{CreateArgs, DeleteArgs, UpdateArgs};

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

                // can move this away to be much cleaner.
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
                        ai::handler::spawn_title_generation_task(
                            state.clone(),
                            args.chat_id.clone(),
                            args.body.clone(),
                            user_id.to_string(),
                        );
                    }
                    ai::handler::spawn_chat_task(
                        state,
                        user_id.to_string(),
                        args.to_owned(),
                        messages,
                    );
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

use anyhow::Result;
use diesel::prelude::*;
use serde::Deserialize;

use crate::app::AppState;
use crate::models::chat::{CreateArgs, DeleteArgs, ForkArgs, UpdateArgs};

use super::handler::Mutation;

#[derive(Debug, Deserialize)]
#[serde(tag = "name", content = "args")]
pub enum ChatMutation {
    #[serde(rename = "createChat")]
    Create(CreateArgs),
    #[serde(rename = "updateChat")]
    Update(UpdateArgs),
    #[serde(rename = "deleteChat")]
    Delete(DeleteArgs),
    #[serde(rename = "forkChat")]
    Fork(ForkArgs),
}

impl ChatMutation {}

impl Mutation for ChatMutation {
    fn process(
        &self,
        state: AppState,
        conn: &mut MysqlConnection,
        user_id: &str,
    ) -> Result<Option<String>> {
        match self {
            ChatMutation::Create(args) => {
                let chat =
                    state
                        .service_container
                        .chat_service
                        .create(conn, args.clone(), user_id)?;
                Ok(Some(chat.id))
            }
            ChatMutation::Update(args) => {
                let chat =
                    state
                        .service_container
                        .chat_service
                        .update(conn, args.clone(), user_id)?;
                Ok(Some(chat.id))
            }
            ChatMutation::Delete(args) => {
                let chat = state
                    .service_container
                    .chat_service
                    .delete(conn, &args.id, user_id)?;
                Ok(Some(chat.id))
            }
            ChatMutation::Fork(args) => {
                let chat = state
                    .service_container
                    .chat_service
                    .fork(conn, args, user_id)?;
                Ok(Some(chat.id))
            }
        }
    }
}

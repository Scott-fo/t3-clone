use anyhow::Result;
use diesel::prelude::*;
use serde::Deserialize;

use crate::app::AppState;
use crate::models::message::{CreateArgs, UpdateArgs};

use super::handler::Mutation;

#[derive(Debug, Deserialize)]
#[serde(tag = "name", content = "args")]
pub enum MessageMutation {
    #[serde(rename = "createMessage")]
    Create(CreateArgs),
    #[serde(rename = "updateMessage")]
    Update(UpdateArgs),
    #[serde(rename = "deleteMessage")]
    Delete(UpdateArgs),
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

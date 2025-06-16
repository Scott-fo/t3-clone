use anyhow::{Context, Result, bail};
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    models::{
        shared_chat::{SharedChat, SharedChatWithMessages},
        shared_message::SharedMessage,
    },
    repositories::{shared_chat::SharedChatRepository, shared_message::SharedMessageRepository},
    services::{chat::ChatService, message::MessageService},
};

#[derive(Debug, Clone)]
pub struct SharedChatService {
    chat_svc: ChatService,
    msg_svc: MessageService,
}

impl SharedChatService {
    pub fn new(chat_svc: ChatService, msg_svc: MessageService) -> Self {
        Self { chat_svc, msg_svc }
    }

    pub fn get(
        &self,
        conn: &mut MysqlConnection,
        shared_chat_id: &str,
    ) -> Result<SharedChatWithMessages> {
        let chat: SharedChat =
            SharedChatRepository::get(conn, shared_chat_id).context("shared chat not found")?;

        let messages: Vec<SharedMessage> =
            SharedMessageRepository::list_for_shared_chat(conn, shared_chat_id)?;

        Ok(SharedChatWithMessages {
            id: chat.id,
            title: chat.title,
            created_at: chat.created_at,
            messages,
        })
    }

    pub fn create(
        &self,
        conn: &mut MysqlConnection,
        chat_id: &str,
        user_id: &str,
    ) -> Result<SharedChatWithMessages> {
        conn.transaction(|tx| {
            let private_chat = self.chat_svc.get(tx, chat_id, user_id)?;
            let private_messages = self.msg_svc.list_for_chat(tx, chat_id, user_id)?;

            let shared_chat_id = Uuid::new_v4().to_string();
            let new_shared_chat = SharedChat {
                id: shared_chat_id.clone(),
                original_chat_id: chat_id.to_owned(),
                owner_user_id: user_id.to_owned(),
                title: private_chat.title.clone(),
                created_at: Utc::now().naive_utc(),
            };
            SharedChatRepository::create(tx, &new_shared_chat)?;

            let new_msgs: Vec<SharedMessage> = private_messages
                .into_iter()
                .map(|m| SharedMessage {
                    id: Uuid::new_v4().to_string(),
                    shared_chat_id: shared_chat_id.clone(),
                    role: m.role,
                    body: m.body,
                    reasoning: m.reasoning,
                    created_at: m.created_at,
                })
                .collect();

            SharedMessageRepository::bulk_create(tx, &new_msgs)?;

            let chat = SharedChatRepository::get(tx, &shared_chat_id)?;
            let msgs = SharedMessageRepository::list_for_shared_chat(tx, &shared_chat_id)?;

            Ok(SharedChatWithMessages {
                id: chat.id,
                title: chat.title,
                created_at: chat.created_at,
                messages: msgs,
            })
        })
    }

    pub fn delete(
        &self,
        conn: &mut MysqlConnection,
        shared_chat_id: &str,
        user_id: &str,
    ) -> Result<()> {
        let chat: SharedChat =
            SharedChatRepository::get(conn, shared_chat_id).context("shared chat not found")?;

        if chat.owner_user_id != user_id {
            bail!("Forbidden: You do not own this shared chat");
        }

        let affected = SharedChatRepository::delete(conn, shared_chat_id, user_id)?;
        anyhow::ensure!(affected == 1, "nothing deleted");

        Ok(())
    }
}

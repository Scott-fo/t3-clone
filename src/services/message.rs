use anyhow::{Context, Result, bail};
use diesel::prelude::*;

use crate::{
    models::message::{Changeset, CreateArgs, Message, UpdateArgs},
    repositories::{Repository, chat::ChatRepository, message::MessageRepository},
};

#[derive(Debug, Clone)]
pub struct MessageService {
    message_repo: MessageRepository,
    chat_repo: ChatRepository,
}

impl MessageService {
    pub fn new(message_repo: MessageRepository, chat_repo: ChatRepository) -> Self {
        Self {
            message_repo,
            chat_repo,
        }
    }

    fn check_ownership(
        &self,
        conn: &mut MysqlConnection,
        message_id: &str,
        user_id: &str,
    ) -> Result<Message> {
        let message = self
            .message_repo
            .find_by_id(conn, message_id)?
            .context(format!("Message {} not found", message_id))?;

        let chat = self
            .chat_repo
            .find_by_id(conn, &message.chat_id)?
            .context(format!(
                "Data integrity error: Chat {} not found for message {}",
                message.chat_id, message.id
            ))?;

        if chat.user_id != user_id {
            bail!("Forbidden: You do not have access to this message.");
        }

        Ok(message)
    }

    pub fn create(
        &self,
        conn: &mut MysqlConnection,
        args: CreateArgs,
        user_id: &str,
    ) -> Result<Message> {
        let chat = self
            .chat_repo
            .find_by_id(conn, &args.chat_id)?
            .context(format!("Chat {} not found", args.chat_id))?;

        if chat.user_id != user_id {
            bail!("Forbidden: You cannot post messages in this chat.");
        }

        let message = Message {
            id: args.id,
            chat_id: args.chat_id,
            user_id: user_id.to_string(),
            body: args.body,
            version: 1,
            created_at: args.created_at,
            updated_at: args.updated_at,
        };

        self.message_repo.create(conn, &message)
    }

    pub fn update(
        &self,
        conn: &mut MysqlConnection,
        args: UpdateArgs,
        user_id: &str,
    ) -> Result<Message> {
        conn.transaction(|conn| {
            let existing = self.check_ownership(conn, &args.id, user_id)?;

            let changeset = Changeset {
                body: args.body,
                version: existing.version + 1,
                updated_at: args.updated_at,
            };

            self.message_repo.update(conn, &args.id, changeset)
        })
    }

    pub fn delete(&self, conn: &mut MysqlConnection, id: &str, user_id: &str) -> Result<Message> {
        conn.transaction(|conn| {
            let message = self.check_ownership(conn, id, user_id)?;

            self.message_repo.delete(conn, id)?;

            Ok(message)
        })
    }

    pub fn list_for_chat(
        &self,
        conn: &mut MysqlConnection,
        chat_id: &str,
        user_id: &str,
    ) -> Result<Vec<Message>> {
        let chat = self
            .chat_repo
            .find_by_id(conn, chat_id)?
            .context(format!("Chat {} not found", chat_id))?;

        if chat.user_id != user_id {
            bail!("Forbidden: You cannot view messages in this chat.");
        }

        self.message_repo.find_by_chat(conn, chat_id)
    }
}

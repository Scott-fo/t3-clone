use anyhow::{Context, Result, bail};
use chrono::Utc;
use diesel::prelude::*;

use crate::{
    ai::{handler::StreamResult, provider::AiProvider},
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
            role: args.role,
            body: args.body,
            reasoning: args.reasoning,
            version: 1,
            created_at: args.created_at.naive_utc(),
            updated_at: args.updated_at.naive_utc(),
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

            let mut changeset = Changeset {
                body: args.body,
                version: existing.version + 1,
                reasoning: None,
                updated_at: args.updated_at.naive_utc(),
            };

            if args.reasoning.is_some() {
                changeset.reasoning = args.reasoning.unwrap();
            }

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

    pub fn save_assistant_reply(
        &self,
        conn: &mut MysqlConnection,
        chat_id: &str,
        reply: StreamResult,
        user_id: &str,
    ) -> Result<Message> {
        let now = Utc::now();

        let args = CreateArgs {
            id: reply.msg_id,
            chat_id: chat_id.to_owned(),
            role: "assistant".to_owned(),
            body: reply.content,
            reasoning: reply.reasoning,
            created_at: now,
            updated_at: now,
        };

        self.create(conn, args, user_id)
    }

    pub fn save_assistant_error(
        &self,
        conn: &mut MysqlConnection,
        chat_id: &str,
        provider: AiProvider,
        user_id: &str,
    ) -> Result<Message> {
        let now = Utc::now();

        let args = CreateArgs {
            id: uuid::Uuid::new_v4().to_string(),
            chat_id: chat_id.to_owned(),
            role: "assistant".into(),
            body: format!("Error: Missing API key for {}", provider),
            reasoning: None,
            created_at: now,
            updated_at: now,
        };

        self.create(conn, args, user_id)
    }
}

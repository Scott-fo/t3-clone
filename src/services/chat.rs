use crate::{
    models::{
        chat::{Changeset, Chat, ChatWithMessages, CreateArgs, UpdateArgs},
        message::Message,
    },
    repositories::{Repository, chat::ChatRepository, message::MessageRepository},
};
use anyhow::{Context, Result, bail};
use diesel::prelude::*;

#[derive(Debug, Clone)]
pub struct ChatService {
    repository: ChatRepository,
    msg_repo: MessageRepository,
}

impl ChatService {
    fn check_ownership(
        &self,
        conn: &mut MysqlConnection,
        chat_id: &str,
        user_id: &str,
    ) -> Result<Chat> {
        let chat = self
            .repository
            .find_by_id(conn, chat_id)?
            .ok_or(anyhow::anyhow!("Failed to find chat"))?;

        if chat.user_id != user_id {
            bail!("Forbidden: You do not have access to this chat.");
        }

        Ok(chat)
    }

    pub fn new(repository: ChatRepository, msg_repo: MessageRepository) -> Self {
        Self {
            repository,
            msg_repo,
        }
    }

    pub fn create(
        &self,
        conn: &mut MysqlConnection,
        args: CreateArgs,
        user_id: &str,
    ) -> Result<Chat> {
        let chat = Chat {
            id: args.id,
            user_id: user_id.to_string(),
            title: None,
            archived: false,
            version: 1,
            created_at: args.created_at,
            updated_at: args.updated_at,
        };

        self.repository.create(conn, &chat)
    }

    pub fn update(
        &self,
        conn: &mut MysqlConnection,
        args: UpdateArgs,
        user_id: &str,
    ) -> Result<Chat> {
        conn.transaction(|conn| {
            let existing = self
                .repository
                .find_by_id_for_update(conn, &args.id)?
                .ok_or_else(|| {
                    anyhow::anyhow!(format!("Failed to find existing chat: {}", args.id))
                })?;

            self.check_ownership(conn, &args.id, user_id)?;

            let changeset = Changeset {
                title: args.title,
                archived: args.archived,
                version: existing.version + 1,
                updated_at: args.updated_at,
            };

            self.repository.update(conn, &args.id, changeset)
        })
    }

    pub fn delete(&self, conn: &mut MysqlConnection, id: &str, user_id: &str) -> Result<Chat> {
        conn.transaction(|conn| {
            let chat = self
                .repository
                .find_by_id_for_update(conn, id)?
                .ok_or_else(|| anyhow::anyhow!(format!("Failed to find chat: {}", id)))?;

            self.check_ownership(conn, id, user_id)?;

            self.msg_repo.delete_by_chat_id(conn, id)?;
            self.repository.delete(conn, id)?;

            Ok(chat)
        })
    }

    pub fn get(&self, conn: &mut MysqlConnection, id: &str, user_id: &str) -> Result<Chat> {
        let chat = self
            .repository
            .find_by_id(conn, id)?
            .ok_or_else(|| anyhow::anyhow!("Failed to find chat: {}", id))?;

        self.check_ownership(conn, id, user_id)?;

        Ok(chat)
    }

    pub fn list_for_user(&self, conn: &mut MysqlConnection, user_id: &str) -> Result<Vec<Chat>> {
        self.repository.find_by_user(conn, user_id)
    }

    pub fn list_with_messages_for_user(
        &self,
        conn: &mut MysqlConnection,
        user_id: &str,
    ) -> Result<Vec<ChatWithMessages>> {
        let chat_repo = ChatRepository::from(self.repository.clone());

        let chats = chat_repo.find_by_user(conn, user_id)?;

        if chats.is_empty() {
            return Ok(Vec::new());
        }

        let messages = Message::belonging_to(&chats)
            .load::<Message>(conn)
            .context(format!("Error loading messages for user {}", user_id))?;

        let chats_with_messages: Vec<(Chat, Vec<Message>)> = messages
            .grouped_by(&chats)
            .into_iter()
            .zip(chats)
            .map(|(messages, chat)| (chat, messages))
            .collect();

        let result = chats_with_messages
            .into_iter()
            .map(|(chat, messages)| ChatWithMessages {
                id: chat.id,
                user_id: chat.user_id,
                title: chat.title,
                archived: chat.archived,
                version: chat.version,
                created_at: chat.created_at,
                updated_at: chat.updated_at,
                messages,
            })
            .collect();

        Ok(result)
    }
}

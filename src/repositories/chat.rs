use anyhow::{Context, Result};
use diesel::BelongingToDsl;
use diesel::prelude::*;

use crate::models::{
    chat::{Changeset, Chat, ChatWithMessages},
    message::Message,
};

use super::Repository;

#[derive(Debug, Clone)]
pub struct ChatRepository;

impl Repository<Chat, Changeset> for ChatRepository {
    fn find_by_id(&self, conn: &mut MysqlConnection, id: &str) -> Result<Option<Chat>> {
        use crate::schema::chats::dsl::chats;

        match chats.find(id).first::<Chat>(conn) {
            Ok(chat) => Ok(Some(chat)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e).context(format!("Error finding chat with id {}", id)),
        }
    }

    fn find_by_ids(&self, conn: &mut MysqlConnection, ids: &[&str]) -> Result<Vec<Chat>> {
        use crate::schema::chats::dsl::{chats, id};

        chats
            .filter(id.eq_any(ids))
            .load(conn)
            .context("Failed to find chats by IDs")
    }

    fn find_by_id_for_update(&self, conn: &mut MysqlConnection, id: &str) -> Result<Option<Chat>> {
        use crate::schema::chats::dsl::chats;

        match chats.find(id).for_update().first::<Chat>(conn) {
            Ok(chat) => Ok(Some(chat)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e).context(format!("Error finding chat {} for update", id)),
        }
    }

    fn find_by_user(&self, conn: &mut MysqlConnection, user_id_param: &str) -> Result<Vec<Chat>> {
        use crate::schema::chats::dsl::{chats, user_id};

        chats
            .filter(user_id.eq(user_id_param))
            .load(conn)
            .context(format!("Error finding chats for user {}", user_id_param))
    }

    fn create(&self, conn: &mut MysqlConnection, entity: &Chat) -> Result<Chat> {
        use crate::schema::chats::dsl::chats;

        diesel::insert_into(chats)
            .values(entity)
            .execute(conn)
            .context(format!("Error creating chat {}", entity.id))?;

        Ok(entity.clone())
    }

    fn update(&self, conn: &mut MysqlConnection, id: &str, changeset: Changeset) -> Result<Chat> {
        use crate::schema::chats::dsl::chats;

        diesel::update(chats.find(id))
            .set(changeset)
            .execute(conn)
            .context(format!("Error updating chat {}", id))?;

        self.find_by_id(conn, id)?
            .context(format!("Chat {} not found after update", id))
    }

    fn delete(&self, conn: &mut MysqlConnection, id: &str) -> Result<()> {
        use crate::schema::chats::dsl::chats;

        diesel::delete(chats.find(id))
            .execute(conn)
            .context(format!("Error deleting chat {}", id))?;

        Ok(())
    }
}

impl ChatRepository {
    pub fn find_with_messages(
        &self,
        conn: &mut MysqlConnection,
        chat_id: &str,
    ) -> Result<Option<ChatWithMessages>> {
        let chat = match self.find_by_id(conn, chat_id)? {
            Some(chat) => chat,
            None => return Ok(None),
        };

        let messages = Message::belonging_to(&chat)
            .load::<Message>(conn)
            .context(format!("Error loading messages for chat {}", chat.id))?;

        let result = ChatWithMessages {
            id: chat.id,
            user_id: chat.user_id,
            title: chat.title,
            pinned: chat.pinned,
            archived: chat.archived,
            forked: chat.forked,
            version: chat.version,
            pinned_at: chat.pinned_at,
            created_at: chat.created_at,
            updated_at: chat.updated_at,
            messages,
        };

        Ok(Some(result))
    }
}

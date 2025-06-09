use anyhow::{Context, Result};
use diesel::prelude::*;

use crate::models::message::{Changeset, Message};

use super::Repository;

#[derive(Debug, Clone)]
pub struct MessageRepository;

impl Repository<Message, Changeset> for MessageRepository {
    fn find_by_id(&self, conn: &mut MysqlConnection, id: &str) -> Result<Option<Message>> {
        use crate::schema::messages::dsl::messages;

        match messages.find(id).first::<Message>(conn) {
            Ok(msg) => Ok(Some(msg)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e).context(format!("Error finding message with id {}", id)),
        }
    }

    fn find_by_ids(&self, conn: &mut MysqlConnection, ids: &[&str]) -> Result<Vec<Message>> {
        use crate::schema::messages::dsl::{id, messages};

        messages
            .filter(id.eq_any(ids))
            .load(conn)
            .context("Failed to find messages by IDs")
    }

    fn find_by_id_for_update(
        &self,
        conn: &mut MysqlConnection,
        id: &str,
    ) -> Result<Option<Message>> {
        use crate::schema::messages::dsl::messages;

        match messages.find(id).for_update().first::<Message>(conn) {
            Ok(msg) => Ok(Some(msg)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e).context(format!("Error finding message {} for update", id)),
        }
    }

    fn find_by_user(
        &self,
        conn: &mut MysqlConnection,
        user_id_param: &str,
    ) -> Result<Vec<Message>> {
        use crate::schema::messages::dsl::{messages, user_id};

        messages
            .filter(user_id.eq(user_id_param))
            .load(conn)
            .context(format!("Error finding messages for user {}", user_id_param))
    }

    fn create(&self, conn: &mut MysqlConnection, entity: &Message) -> Result<Message> {
        use crate::schema::messages::dsl::messages;

        diesel::insert_or_ignore_into(messages)
            .values(entity)
            .execute(conn)
            .context(format!("Error creating message {}", entity.id))?;

        Ok(entity.clone())
    }

    fn update(
        &self,
        conn: &mut MysqlConnection,
        id: &str,
        changeset: Changeset,
    ) -> Result<Message> {
        use crate::schema::messages::dsl::messages;

        diesel::update(messages.find(id))
            .set(changeset)
            .execute(conn)
            .context(format!("Error updating message {}", id))?;

        self.find_by_id(conn, id)?
            .context(format!("Message {} not found after update", id))
    }

    fn delete(&self, conn: &mut MysqlConnection, id: &str) -> Result<()> {
        use crate::schema::messages::dsl::messages;

        diesel::delete(messages.find(id))
            .execute(conn)
            .context(format!("Error deleting message {}", id))?;

        Ok(())
    }
}

impl MessageRepository {
    pub fn find_by_chat(
        &self,
        conn: &mut MysqlConnection,
        chat_id_param: &str,
    ) -> Result<Vec<Message>> {
        use crate::schema::messages::dsl::{chat_id, messages};

        messages
            .filter(chat_id.eq(chat_id_param))
            .load(conn)
            .context(format!("Error finding messages for chat {}", chat_id_param))
    }

    pub fn delete_by_chat_id(&self, conn: &mut MysqlConnection, chat_id_param: &str) -> Result<()> {
        use crate::schema::messages::dsl::{chat_id, messages};

        diesel::delete(messages.filter(chat_id.eq(chat_id_param)))
            .execute(conn)
            .context(format!(
                "Error deleting messages for chat {}",
                chat_id_param
            ))?;

        Ok(())
    }
}

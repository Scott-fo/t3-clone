use anyhow::Result;
use diesel::{RunQueryDsl, prelude::*};

use crate::{models::shared_chat::SharedChat, schema::shared_chats};

#[derive(Debug, Clone)]
pub struct SharedChatRepository;

impl SharedChatRepository {
    pub fn create(conn: &mut MysqlConnection, new: &SharedChat) -> Result<SharedChat> {
        diesel::insert_into(shared_chats::table)
            .values(new)
            .execute(conn)?;

        Ok(shared_chats::table
            .find(&new.id)
            .first::<SharedChat>(conn)?)
    }

    pub fn get(conn: &mut MysqlConnection, id: &str) -> Result<SharedChat> {
        Ok(shared_chats::table.find(id).first::<SharedChat>(conn)?)
    }

    pub fn list_for_user(
        conn: &mut MysqlConnection,
        owner_user_id: &str,
    ) -> Result<Vec<SharedChat>> {
        Ok(shared_chats::table
            .filter(shared_chats::owner_user_id.eq(owner_user_id))
            .load::<SharedChat>(conn)?)
    }

    pub fn delete(conn: &mut MysqlConnection, id: &str, owner_user_id: &str) -> Result<usize> {
        Ok(diesel::delete(
            shared_chats::table
                .filter(shared_chats::id.eq(id))
                .filter(shared_chats::owner_user_id.eq(owner_user_id)),
        )
        .execute(conn)?)
    }
}

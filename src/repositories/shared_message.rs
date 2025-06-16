use anyhow::Result;
use diesel::{RunQueryDsl, prelude::*};

use crate::{models::shared_message::SharedMessage, schema::shared_messages};

#[derive(Debug, Clone)]
pub struct SharedMessageRepository;

impl SharedMessageRepository {
    pub fn create(conn: &mut MysqlConnection, new: &SharedMessage) -> Result<SharedMessage> {
        diesel::insert_into(shared_messages::table)
            .values(new)
            .execute(conn)?;

        Ok(shared_messages::table
            .find(&new.id)
            .first::<SharedMessage>(conn)?)
    }

    pub fn bulk_create(conn: &mut MysqlConnection, msgs: &[SharedMessage]) -> Result<usize> {
        Ok(diesel::insert_into(shared_messages::table)
            .values(msgs)
            .execute(conn)?)
    }

    pub fn list_for_shared_chat(
        conn: &mut MysqlConnection,
        shared_chat_id: &str,
    ) -> Result<Vec<SharedMessage>> {
        Ok(shared_messages::table
            .filter(shared_messages::shared_chat_id.eq(shared_chat_id))
            .order(shared_messages::created_at.asc())
            .load::<SharedMessage>(conn)?)
    }

    pub fn delete_for_shared_chat(
        conn: &mut MysqlConnection,
        shared_chat_id: &str,
    ) -> Result<usize> {
        Ok(diesel::delete(
            shared_messages::table.filter(shared_messages::shared_chat_id.eq(shared_chat_id)),
        )
        .execute(conn)?)
    }
}

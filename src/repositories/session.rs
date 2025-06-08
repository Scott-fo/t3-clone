use anyhow::{Context, Result};
use diesel::prelude::*;

use crate::models::session::{Changeset, Session};

use super::Repository;

#[derive(Debug, Clone)]
pub struct SessionRepository;

impl Repository<Session, Changeset> for SessionRepository {
    fn find_by_id(&self, conn: &mut MysqlConnection, id: &str) -> Result<Option<Session>> {
        use crate::schema::sessions::dsl::sessions;

        match sessions.find(id).first::<Session>(conn) {
            Ok(session) => Ok(Some(session)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e).context(format!("Error finding session with id {}", id)),
        }
    }

    fn find_by_ids(&self, conn: &mut MysqlConnection, ids: &[&str]) -> Result<Vec<Session>> {
        use crate::schema::sessions::dsl::{id, sessions};

        sessions
            .filter(id.eq_any(ids))
            .load(conn)
            .context("Failed to find sessions by IDs")
    }

    fn find_by_id_for_update(
        &self,
        conn: &mut MysqlConnection,
        id: &str,
    ) -> Result<Option<Session>> {
        use crate::schema::sessions::dsl::sessions;

        match sessions.find(id).for_update().first::<Session>(conn) {
            Ok(session) => Ok(Some(session)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e).context(format!("Error finding session {} for update", id)),
        }
    }

    fn find_by_user(
        &self,
        conn: &mut MysqlConnection,
        user_id_param: &str,
    ) -> Result<Vec<Session>> {
        use crate::schema::sessions::dsl::{sessions, user_id};

        sessions
            .filter(user_id.eq(user_id_param))
            .load(conn)
            .context(format!("Error finding sessions for user {}", user_id_param))
    }

    fn create(&self, conn: &mut MysqlConnection, entity: &Session) -> Result<Session> {
        use crate::schema::sessions::dsl::sessions;

        diesel::insert_into(sessions)
            .values(entity)
            .execute(conn)
            .context(format!("Error creating session {}", entity.id))?;

        Ok(entity.clone())
    }

    fn update(
        &self,
        conn: &mut MysqlConnection,
        id: &str,
        changeset: Changeset,
    ) -> Result<Session> {
        use crate::schema::sessions::dsl::sessions;

        diesel::update(sessions.find(id))
            .set(changeset)
            .execute(conn)
            .context(format!("Error updating session {}", id))?;

        self.find_by_id(conn, id)?
            .context(format!("Session {} not found after update", id))
    }

    fn delete(&self, conn: &mut MysqlConnection, id: &str) -> Result<()> {
        use crate::schema::sessions::dsl::sessions;

        diesel::delete(sessions.find(id))
            .execute(conn)
            .context(format!("Error deleting session {}", id))?;

        Ok(())
    }
}

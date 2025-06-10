use anyhow::{Context, Result};
use diesel::prelude::*;

use crate::models::active_model::{ActiveModel, Changeset};

use super::Repository;

#[derive(Debug, Clone)]
pub struct ActiveModelRepository;

impl Repository<ActiveModel, Changeset> for ActiveModelRepository {
    fn find_by_id(&self, conn: &mut MysqlConnection, id: &str) -> Result<Option<ActiveModel>> {
        use crate::schema::active_models::dsl::active_models;

        match active_models.find(id).first::<ActiveModel>(conn) {
            Ok(am) => Ok(Some(am)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e).context(format!("Error finding active model with id {}", id)),
        }
    }

    fn find_by_ids(&self, conn: &mut MysqlConnection, ids: &[&str]) -> Result<Vec<ActiveModel>> {
        use crate::schema::active_models::dsl::{active_models, id};

        active_models
            .filter(id.eq_any(ids))
            .load(conn)
            .context("Failed to find active models by IDs")
    }

    fn find_by_id_for_update(
        &self,
        conn: &mut MysqlConnection,
        id: &str,
    ) -> Result<Option<ActiveModel>> {
        use crate::schema::active_models::dsl::active_models;

        match active_models
            .find(id)
            .for_update()
            .first::<ActiveModel>(conn)
        {
            Ok(am) => Ok(Some(am)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e).context(format!("Error finding active model {} for update", id)),
        }
    }

    fn find_by_user(
        &self,
        conn: &mut MysqlConnection,
        user_id_param: &str,
    ) -> Result<Vec<ActiveModel>> {
        use crate::schema::active_models::dsl::{active_models, user_id};

        active_models
            .filter(user_id.eq(user_id_param))
            .load(conn)
            .context(format!(
                "Error finding active models for user {}",
                user_id_param
            ))
    }

    fn create(&self, conn: &mut MysqlConnection, entity: &ActiveModel) -> Result<ActiveModel> {
        use crate::schema::active_models::dsl::active_models;

        diesel::insert_into(active_models)
            .values(entity)
            .execute(conn)
            .context(format!("Error creating active model {}", entity.id))?;

        Ok(entity.clone())
    }

    fn update(
        &self,
        conn: &mut MysqlConnection,
        id: &str,
        changeset: Changeset,
    ) -> Result<ActiveModel> {
        use crate::schema::active_models::dsl::active_models;

        diesel::update(active_models.find(id))
            .set(changeset)
            .execute(conn)
            .context(format!("Error updating active model {}", id))?;

        self.find_by_id(conn, id)?
            .context(format!("ActiveModel {} not found after update", id))
    }

    fn delete(&self, conn: &mut MysqlConnection, id: &str) -> Result<()> {
        use crate::schema::active_models::dsl::active_models;

        diesel::delete(active_models.find(id))
            .execute(conn)
            .context(format!("Error deleting active model {}", id))?;

        Ok(())
    }
}

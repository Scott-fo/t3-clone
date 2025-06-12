use crate::{
    models::active_model::{ActiveModel, Changeset, CreateArgs, UpdateArgs},
    repositories::{Repository, active_model::ActiveModelRepository},
};
use anyhow::{Result, bail};
use diesel::prelude::*;

#[derive(Debug, Clone)]
pub struct ActiveModelService {
    repository: ActiveModelRepository,
}

impl ActiveModelService {
    fn check_ownership(
        &self,
        conn: &mut MysqlConnection,
        id: &str,
        user_id: &str,
    ) -> Result<ActiveModel> {
        let active_model = self
            .repository
            .find_by_id(conn, id)?
            .ok_or(anyhow::anyhow!("Failed to find active_model"))?;

        if active_model.user_id != user_id {
            bail!("Forbidden: You do not have access to this active model.");
        }

        Ok(active_model)
    }

    pub fn new(repository: ActiveModelRepository) -> Self {
        Self { repository }
    }

    pub fn create(
        &self,
        conn: &mut MysqlConnection,
        args: CreateArgs,
        user_id: &str,
    ) -> Result<ActiveModel> {
        let mut reasoning = None;
        if let Some(r) = args.reasoning {
            reasoning = Some(r.to_string());
        }

        let active_model = ActiveModel {
            id: args.id,
            user_id: user_id.to_string(),
            provider: args.provider,
            model: args.model,
            reasoning,
            version: 1,
            created_at: args.created_at.naive_utc(),
            updated_at: args.updated_at.naive_utc(),
        };

        self.repository.create(conn, &active_model)
    }

    pub fn update(
        &self,
        conn: &mut MysqlConnection,
        args: UpdateArgs,
        user_id: &str,
    ) -> Result<ActiveModel> {
        conn.transaction(|conn| {
            let existing = self
                .repository
                .find_by_id_for_update(conn, &args.id)?
                .ok_or_else(|| {
                    anyhow::anyhow!(format!("Failed to find existing active_model: {}", args.id))
                })?;

            self.check_ownership(conn, &args.id, user_id)?;

            let mut reasoning = None;
            if let Some(r) = args.reasoning {
                reasoning = Some(r.to_string());
            }

            let changeset = Changeset {
                provider: args.provider,
                model: args.model,
                reasoning,
                version: existing.version + 1,
                updated_at: args.updated_at.naive_utc(),
            };

            self.repository.update(conn, &args.id, changeset)
        })
    }

    pub fn delete(
        &self,
        conn: &mut MysqlConnection,
        id: &str,
        user_id: &str,
    ) -> Result<ActiveModel> {
        conn.transaction(|conn| {
            let active_model = self
                .repository
                .find_by_id_for_update(conn, id)?
                .ok_or_else(|| anyhow::anyhow!(format!("Failed to find active_model: {}", id)))?;

            self.check_ownership(conn, id, user_id)?;

            self.repository.delete(conn, id)?;

            Ok(active_model)
        })
    }

    pub fn get(&self, conn: &mut MysqlConnection, id: &str, user_id: &str) -> Result<ActiveModel> {
        let active_model = self
            .repository
            .find_by_id(conn, id)?
            .ok_or_else(|| anyhow::anyhow!("Failed to find active_model: {}", id))?;

        self.check_ownership(conn, id, user_id)?;

        Ok(active_model)
    }

    pub fn list_for_user(
        &self,
        conn: &mut MysqlConnection,
        user_id: &str,
    ) -> Result<Vec<ActiveModel>> {
        self.repository.find_by_user(conn, user_id)
    }

    pub fn get_for_user(
        &self,
        conn: &mut MysqlConnection,
        user_id: &str,
    ) -> Result<Option<ActiveModel>> {
        let list = self.repository.find_by_user(conn, user_id)?;
        if list.len() > 0 {
            return Ok(list.first().cloned());
        }

        Ok(None)
    }
}

use anyhow::{Context, Result};
use diesel::prelude::*;
use secrecy::SecretString;

use crate::{
    models::api_key::{ApiKey, CreateArgs},
    repositories::api_key::ApiKeyRepository,
};

#[derive(Debug, Clone)]
pub struct ApiKeyService {
    master_key: SecretString,
}

impl ApiKeyService {
    pub fn new(master_key: SecretString) -> Self {
        Self { master_key }
    }

    pub fn create(
        &self,
        conn: &mut MysqlConnection,
        user_id: &str,
        args: CreateArgs,
    ) -> Result<ApiKey> {
        let new_api_key = ApiKey::build_new(user_id.to_owned(), args, self.master_key.clone())
            .context("encrypt")?;
        ApiKeyRepository::create(conn, &new_api_key)
    }

    pub fn list(&self, conn: &mut MysqlConnection, user_id: &str) -> Result<Vec<ApiKey>> {
        ApiKeyRepository::list_for_user(conn, user_id)
    }

    pub fn delete(&self, conn: &mut MysqlConnection, id: u64, user_id: &str) -> Result<()> {
        let affected = ApiKeyRepository::delete(conn, id, user_id)?;
        anyhow::ensure!(affected == 1, "nothing deleted");
        Ok(())
    }
}

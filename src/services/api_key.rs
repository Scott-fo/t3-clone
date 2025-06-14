use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose};
use diesel::prelude::*;
use hkdf::Hkdf;
use secrecy::{ExposeSecret, SecretSlice, SecretString};
use sha2::Sha256;

use crate::{
    models::api_key::{ApiKey, CreateArgs},
    repositories::api_key::ApiKeyRepository,
};

#[derive(Debug, Clone)]
pub struct ApiKeyService {
    master_key: SecretSlice<u8>,
}

impl ApiKeyService {
    pub fn new(master_key: SecretString) -> Self {
        let key = parse_master_key(master_key.expose_secret()).expect("Failed to parse master key");
        Self { master_key: key }
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

    pub fn get_and_decrypt(
        &self,
        conn: &mut MysqlConnection,
        user_id: &str,
        provider: &str,
    ) -> Result<SecretString> {
        self.get_for_provider(conn, user_id, provider)
            .and_then(|key| self.decrypt(key))
    }

    pub fn get_for_provider(
        &self,
        conn: &mut MysqlConnection,
        user_id: &str,
        provider: &str,
    ) -> Result<ApiKey> {
        ApiKeyRepository::get_for_provider(conn, user_id, provider)
    }

    pub fn decrypt(&self, key: ApiKey) -> Result<SecretString> {
        key.decrypt(&self.master_key)
    }

    pub fn delete(&self, conn: &mut MysqlConnection, id: u64, user_id: &str) -> Result<()> {
        let affected = ApiKeyRepository::delete(conn, id, user_id)?;
        anyhow::ensure!(affected == 1, "nothing deleted");
        Ok(())
    }
}

fn parse_master_key(b64: &str) -> Result<SecretSlice<u8>> {
    let decoded = general_purpose::STANDARD
        .decode(b64.trim())
        .context("MASTER_KEY is not valid base-64")?;

    anyhow::ensure!(
        decoded.len() == 64,
        "MASTER_KEY must decode to 64 bytes, got {}",
        decoded.len()
    );

    let hk = Hkdf::<Sha256>::new(None, &decoded);
    let mut hmac_key = [0u8; 32];

    hk.expand(b"hmac-sha-256 key", &mut hmac_key)
        .map_err(|_| anyhow::anyhow!("Failed to expand HMAC key"))?;

    Ok(hmac_key.to_vec().into())
}

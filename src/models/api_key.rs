use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
use anyhow::{Result, anyhow};
use rand::RngCore;

use crate::dtos;

const NONCE_LEN: usize = 12;

#[derive(Queryable, Identifiable, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::api_keys)]
pub struct ApiKey {
    pub id: u64,
    pub user_id: String,
    pub provider: String,
    pub encrypted_key: Vec<u8>,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::api_keys)]
pub struct NewApiKey {
    pub user_id: String,
    pub provider: String,
    pub encrypted_key: Vec<u8>,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateArgs {
    pub provider: String,
    #[serde(rename = "key")]
    pub api_key: SecretString,
}

impl ApiKey {
    pub fn encrypt_plain_text(plain: &SecretString, master_key: &SecretString) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new(master_key.expose_secret().as_bytes().into());

        let mut nonce_bytes = [0u8; NONCE_LEN];
        rand::rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let mut out = nonce_bytes.to_vec();
        let mut ct = cipher
            .encrypt(nonce, plain.expose_secret().as_bytes())
            .map_err(|_| anyhow::anyhow!("Failed to encrypt api key"))?;

        out.append(&mut ct);
        Ok(out)
    }

    pub fn decrypt(&self, master_key: &SecretString) -> Result<SecretString> {
        if self.encrypted_key.len() <= NONCE_LEN {
            return Err(anyhow!("ciphertext too short"));
        }

        let (nonce_bytes, ct) = self.encrypted_key.split_at(NONCE_LEN);
        let cipher = Aes256Gcm::new(master_key.expose_secret().as_bytes().into());
        let plain = cipher
            .decrypt(Nonce::from_slice(nonce_bytes), ct)
            .map_err(|_| anyhow::anyhow!("Failed to decrypt api key"))?;

        Ok(SecretString::new(
            String::from_utf8_lossy(&plain).into_owned().into(),
        ))
    }

    pub fn build_new(
        user_id: String,
        args: CreateArgs,
        master_key: SecretString,
    ) -> Result<NewApiKey> {
        Ok(NewApiKey {
            user_id,
            provider: args.provider,
            encrypted_key: Self::encrypt_plain_text(&args.api_key, &master_key)?,
            version: 1,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        })
    }
}

impl From<ApiKey> for dtos::api_key::ApiKey {
    fn from(value: ApiKey) -> Self {
        Self {
            id: value.id,
            provider: value.provider,
            created_at: value.created_at.and_utc(),
            updated_at: value.updated_at.and_utc(),
        }
    }
}

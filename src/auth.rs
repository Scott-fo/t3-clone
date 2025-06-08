use anyhow::{Context, Result};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use secrecy::{ExposeSecret, SecretString};

pub fn hash_password(password: &SecretString) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    let params = argon2::Params::new(128 * 1024, 4, 1, Some(32))
        .context("Failed to create Argon2 params")?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    let password_hash = argon2
        .hash_password(password.expose_secret().as_bytes(), &salt)
        .context("Failed to hash password")?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &SecretString, hash: &str) -> Result<()> {
    let parsed_hash =
        PasswordHash::new(hash).context("Failed to parse password hash from database")?;

    Argon2::default()
        .verify_password(password.expose_secret().as_bytes(), &parsed_hash)
        .context("Password verification failed")?;

    Ok(())
}

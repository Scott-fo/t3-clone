use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::{auth, infra::db::DbPool};
use anyhow::{Context, Result, bail};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: SecretString,
    pub password_confirmation: SecretString,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_digest: String,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn new(new_user: NewUser, hash: String) -> Self {
        let now = Utc::now().naive_utc();

        Self {
            id: Uuid::new_v4().to_string(),
            email: new_user.email,
            password_digest: hash,
            version: 1,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn create(new_user: NewUser, conn: &mut MysqlConnection) -> Result<Self> {
        if new_user.password.expose_secret().len() < 12 {
            bail!("Password should be at least 12 characters");
        }
        if new_user.password.expose_secret() != new_user.password_confirmation.expose_secret() {
            bail!("Passwords do not match");
        }

        match users.filter(email.eq(&new_user.email)).first::<User>(conn) {
            Ok(_) => bail!("Email already exists"),
            Err(diesel::result::Error::NotFound) => (),
            Err(e) => return Err(e).context("Database check for existing email failed"),
        }

        let password_hash = auth::hash_password(&new_user.password)?;

        let user = User::new(new_user, password_hash);

        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .context("Error inserting new user into the database")?;

        Ok(user)
    }

    pub fn authenticate(user_email: &str, password: &SecretString, pool: &DbPool) -> Result<User> {
        let mut conn = pool
            .get()
            .context("Failed to get DB connection from pool")?;

        let user = users
            .filter(email.eq(user_email))
            .first::<User>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    anyhow::anyhow!("Invalid email or password")
                }
                _ => anyhow::Error::new(e).context(format!(
                    "Database error while finding user with email {}",
                    user_email
                )),
            })?;

        auth::verify_password(password, &user.password_digest)
            .map_err(|_| anyhow::anyhow!("Invalid email or password"))?;

        Ok(user)
    }
}

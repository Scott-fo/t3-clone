use anyhow::Result;
use diesel::{RunQueryDsl, prelude::*};

use crate::models::api_key::{ApiKey, NewApiKey};
use crate::schema::api_keys;

pub struct ApiKeyRepository;

impl ApiKeyRepository {
    pub fn create(conn: &mut MysqlConnection, new: &NewApiKey) -> Result<ApiKey> {
        diesel::insert_into(api_keys::table)
            .values(new)
            .execute(conn)?;

        let inserted = api_keys::table
            .order(api_keys::id.desc())
            .first::<ApiKey>(conn)?;
        Ok(inserted)
    }

    pub fn list_for_user(conn: &mut MysqlConnection, user_id: &str) -> Result<Vec<ApiKey>> {
        Ok(api_keys::table
            .filter(api_keys::user_id.eq(user_id))
            .load::<ApiKey>(conn)?)
    }

    pub fn delete(conn: &mut MysqlConnection, id: u64, user_id: &str) -> Result<usize> {
        Ok(diesel::delete(
            api_keys::table
                .filter(api_keys::id.eq(id))
                .filter(api_keys::user_id.eq(user_id)),
        )
        .execute(conn)?)
    }
}

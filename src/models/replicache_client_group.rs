use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use crate::models::user::User;

#[derive(Debug, Queryable, Insertable, Associations, Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::replicache_client_groups)]
pub struct ReplicacheClientGroup {
    pub id: String,
    pub user_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub cvr_version: i32,
}

impl ReplicacheClientGroup {
    pub fn new(id: String, user_id: String) -> Self {
        let now = Utc::now().naive_utc();

        Self {
            id,
            user_id,
            created_at: now,
            updated_at: now,
            cvr_version: 0,
        }
    }
}

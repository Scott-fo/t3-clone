use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{AsChangeset, Associations, Insertable, Queryable};
use uuid::Uuid;

use crate::models::user::User;

#[derive(Clone, Queryable, Insertable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::sessions)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub expired_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::sessions)]
pub struct Changeset {
    expired_at: Option<NaiveDateTime>,
}

impl Session {
    pub fn new(user_id: &str) -> Self {
        let now = Utc::now().naive_utc();

        Self {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            expired_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}

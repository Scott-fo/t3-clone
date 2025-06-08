use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::chat::Chat;

#[derive(
    Debug,
    PartialEq,
    Identifiable,
    Associations,
    Queryable,
    Insertable,
    Serialize,
    Deserialize,
    Clone,
)]
#[diesel(belongs_to(Chat))]
#[diesel(table_name = crate::schema::messages)]
pub struct Message {
    pub id: String,
    pub chat_id: String,
    pub user_id: String,
    pub body: String,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::messages)]
pub struct Changeset {
    body: Option<String>,
    version: i32,
    updated_at: NaiveDateTime,
}

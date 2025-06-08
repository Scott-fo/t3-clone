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
    pub body: Option<String>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateArgs {
    pub id: String,
    pub chat_id: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArgs {
    pub id: String,
    pub body: Option<String>,
    pub updated_at: NaiveDateTime,
}

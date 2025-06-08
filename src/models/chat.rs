use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::message::Message;

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::chats)]
pub struct Chat {
    pub id: String,
    pub user_id: String,
    pub title: Option<String>,
    pub archived: bool,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::chats)]
pub struct Changeset {
    pub title: Option<String>,
    pub archived: Option<bool>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
}

pub struct CreateArgs {
    pub id: String,
    pub user_id: String,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct UpdateArgs {
    pub id: String,
    pub title: Option<String>,
    pub archived: Option<bool>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct ChatWithMessages {
    pub id: String,
    pub user_id: String,
    pub title: Option<String>,
    pub archived: bool,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub messages: Vec<Message>,
}

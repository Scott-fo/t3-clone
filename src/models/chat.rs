use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::dtos;

use super::{message::Message, replicache::ReplicachePullModel};

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::chats)]
pub struct Chat {
    pub id: String,
    pub user_id: String,
    pub title: Option<String>,
    pub archived: bool,
    pub pinned: bool,
    pub forked: bool,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::chats)]
pub struct Changeset {
    pub title: Option<String>,
    pub archived: Option<bool>,
    pub pinned: Option<bool>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArgs {
    pub id: String,
    pub user_id: String,
    pub version: i32,
    pub forked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateArgs {
    pub id: String,
    pub title: Option<String>,
    pub archived: Option<bool>,
    pub pinned: Option<bool>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteArgs {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ForkArgs {
    pub new_id: String,
    pub title: String,
    pub time: DateTime<Utc>,
    pub msgs: Vec<super::message::CreateArgs>,
}

#[derive(Debug, Serialize)]
pub struct ChatWithMessages {
    pub id: String,
    pub user_id: String,
    pub title: Option<String>,
    pub pinned: bool,
    pub archived: bool,
    pub forked: bool,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub messages: Vec<Message>,
}

impl ReplicachePullModel for Chat {
    fn resource_prefix() -> &'static str {
        "chat"
    }

    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_version(&self) -> i32 {
        self.version
    }
}

impl From<Chat> for dtos::chat::Chat {
    fn from(value: Chat) -> Self {
        dtos::chat::Chat {
            id: value.id,
            title: value.title,
            pinned: value.pinned,
            archived: value.archived,
            forked: value.forked,
            created_at: value.created_at.and_utc(),
            updated_at: value.updated_at.and_utc(),
        }
    }
}

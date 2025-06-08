use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::{message::Message, replicache::ReplicachePullModel};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArgs {
    pub id: String,
    pub user_id: String,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateArgs {
    pub id: String,
    pub title: Option<String>,
    pub archived: Option<bool>,
    pub updated_at: DateTime<Utc>,
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

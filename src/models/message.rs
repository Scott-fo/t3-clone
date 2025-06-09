use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{dtos, models::chat::Chat};

use super::replicache::ReplicachePullModel;

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
    pub role: String,
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

#[derive(Debug, Clone, Deserialize)]
pub struct CreateArgs {
    pub id: String,
    pub chat_id: String,
    pub role: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateArgs {
    pub id: String,
    pub body: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteArgs {
    pub id: String,
}

impl ReplicachePullModel for Message {
    fn resource_prefix() -> &'static str {
        "message"
    }

    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_version(&self) -> i32 {
        self.version
    }
}

impl From<Message> for dtos::message::Message {
    fn from(value: Message) -> Self {
        dtos::message::Message {
            id: value.id,
            chat_id: value.chat_id,
            role: value.role,
            body: value.body,
            created_at: value.created_at.and_utc(),
            updated_at: value.updated_at.and_utc(),
        }
    }
}

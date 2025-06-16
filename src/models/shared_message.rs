use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::dtos;
use crate::models::shared_chat::SharedChat;

#[derive(
    Debug, Queryable, Identifiable, Associations, Insertable, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema::shared_messages)]
#[diesel(belongs_to(SharedChat, foreign_key = shared_chat_id))]
pub struct SharedMessage {
    pub id: String,
    pub shared_chat_id: String,
    pub role: String,
    pub body: String,
    pub reasoning: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedMessageCreateArgs {
    pub id: String,
    pub shared_chat_id: String,
    pub role: String,
    pub body: String,
    pub reasoning: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<super::shared_message::SharedMessage> for dtos::shared_chat::SharedMessage {
    fn from(src: super::shared_message::SharedMessage) -> Self {
        dtos::shared_chat::SharedMessage {
            id: src.id,
            role: src.role,
            body: src.body,
            reasoning: src.reasoning,
            created_at: src.created_at.and_utc(),
        }
    }
}

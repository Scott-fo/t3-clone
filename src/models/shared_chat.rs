use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::dtos;

use super::shared_message::SharedMessage;

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::shared_chats)]
pub struct SharedChat {
    pub id: String,
    pub original_chat_id: String,
    pub owner_user_id: String,
    pub title: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::shared_chats)]
pub struct SharedChatChangeset {
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedChatCreateArgs {
    pub id: String,
    pub original_chat_id: String,
    pub owner_user_id: String,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedChatDeleteArgs {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct SharedChatWithMessages {
    pub id: String,
    pub title: Option<String>,
    pub created_at: NaiveDateTime,
    pub messages: Vec<SharedMessage>,
}

impl From<SharedChat> for dtos::shared_chat::SharedChat {
    fn from(value: SharedChat) -> Self {
        dtos::shared_chat::SharedChat {
            id: value.id,
            title: value.title,
            created_at: value.created_at.and_utc(),
        }
    }
}

impl From<SharedChatWithMessages> for dtos::shared_chat::SharedChatWithMessages {
    fn from(src: SharedChatWithMessages) -> Self {
        dtos::shared_chat::SharedChatWithMessages {
            id: src.id,
            title: src.title,
            created_at: src.created_at.and_utc(),
            messages: src
                .messages
                .into_iter()
                .map(dtos::shared_chat::SharedMessage::from)
                .collect(),
        }
    }
}

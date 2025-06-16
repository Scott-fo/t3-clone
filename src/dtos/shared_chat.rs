use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct SharedChat {
    pub id: String,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct SharedMessage {
    pub id: String,
    pub role: String,
    pub body: String,
    pub reasoning: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct SharedChatWithMessages {
    pub id: String,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub messages: Vec<SharedMessage>,
}

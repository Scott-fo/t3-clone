use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub id: String,
    pub chat_id: String,
    pub role: String,
    pub body: String,
    pub reasoning: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct ActiveModel {
    pub id: String,
    pub provider: String,
    pub model: String,
    pub reasoning: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

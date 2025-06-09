use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct Chat {
    pub id: String,
    pub title: Option<String>,
    pub pinned: bool,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

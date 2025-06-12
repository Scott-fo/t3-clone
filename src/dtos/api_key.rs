use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiKey {
    pub id: u64,
    pub provider: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{ai::openai::Reasoning, dtos};

use super::replicache::ReplicachePullModel;

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::active_models)]
pub struct ActiveModel {
    pub id: String,
    pub user_id: String,
    pub provider: String,
    pub model: String,
    pub reasoning: Option<String>,
    pub version: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::active_models)]
pub struct Changeset {
    pub provider: String,
    pub model: String,
    pub reasoning: Option<String>,
    pub version: i32,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArgs {
    pub id: String,
    pub provider: String,
    pub model: String,
    pub reasoning: Option<Reasoning>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateArgs {
    pub id: String,
    pub provider: String,
    pub model: String,
    pub reasoning: Option<Reasoning>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteArgs {
    pub id: String,
}

impl ReplicachePullModel for ActiveModel {
    fn resource_prefix() -> &'static str {
        "activeModel"
    }

    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_version(&self) -> i32 {
        self.version
    }
}

impl From<ActiveModel> for dtos::active_model::ActiveModel {
    fn from(value: ActiveModel) -> Self {
        dtos::active_model::ActiveModel {
            id: value.id,
            provider: value.provider,
            model: value.model,
            reasoning: value.reasoning,
            created_at: value.created_at.and_utc(),
            updated_at: value.updated_at.and_utc(),
        }
    }
}

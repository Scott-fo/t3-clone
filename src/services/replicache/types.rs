use serde::{Deserialize, Serialize};

use super::cvr::CvrRecord;
use crate::mutations::handler::RawMutation;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cookie {
    pub order: i32,
    #[serde(rename = "cvrID")]
    pub cvr_id: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    #[serde(rename = "clientGroupID")]
    pub client_group_id: String,
    pub cookie: Option<Cookie>,
}

#[derive(Debug, Serialize)]
pub struct PullResponse {
    pub cookie: Cookie,
    #[serde(rename = "lastMutationIDChanges")]
    pub last_mutation_id_changes: std::collections::HashMap<String, i32>,
    pub patch: Vec<PatchOperation>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "op")]
pub enum PatchOperation {
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "del")]
    Delete { key: String },
    #[serde(rename = "put")]
    Put {
        key: String,
        value: serde_json::Value,
    },
}

pub enum PullResult {
    Unchanged {
        cvr_version: i32,
    },
    Changed {
        next_cvr: CvrRecord,
        patch: Vec<PatchOperation>,
        cvr_version: i32,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushRequest {
    #[serde(rename = "clientGroupID")]
    pub client_group_id: String,
    pub mutations: Vec<RawMutation>,
}

#[derive(Debug, Serialize)]
pub struct PushResponse {
    pub success: bool,
}

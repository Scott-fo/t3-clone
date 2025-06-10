use anyhow::Result;
use diesel::prelude::*;
use serde::{Deserialize, Serialize, de::Error};
use serde_json::json;

use crate::app::AppState;

use super::{active_model::ActiveModelMutation, chat::ChatMutation, message::MessageMutation};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RawMutation {
    #[serde(rename = "clientID")]
    pub client_id: String,
    pub id: i32,
    pub name: String,
    pub args: serde_json::Value,
    pub timestamp: f64,
}

pub trait Mutation {
    fn process(
        &self,
        state: AppState,
        conn: &mut MysqlConnection,
        user_id: &str,
    ) -> Result<Option<String>>;
}

pub fn parse_mutation(raw: RawMutation) -> Result<Box<dyn Mutation>, serde_json::Error> {
    match raw.name.as_str() {
        "createChat" | "updateChat" | "deleteChat" => {
            let chat_mutation: ChatMutation = serde_json::from_value(json!({
                "name": raw.name,
                "args": raw.args
            }))?;
            Ok(Box::new(chat_mutation))
        }
        "createMessage" | "updateMessage" | "deleteMessage" => {
            let msg_mutation: MessageMutation = serde_json::from_value(json!({
                "name": raw.name,
                "args": raw.args
            }))?;
            Ok(Box::new(msg_mutation))
        }
        "createActiveModel" | "updateActiveModel" | "deleteActiveModel" => {
            let active_model_mutation: ActiveModelMutation = serde_json::from_value(json!({
                "name": raw.name,
                "args": raw.args
            }))?;
            Ok(Box::new(active_model_mutation))
        }
        _ => Err(serde_json::Error::custom(format!(
            "Unknown mutation type: {}",
            raw.name
        ))),
    }
}

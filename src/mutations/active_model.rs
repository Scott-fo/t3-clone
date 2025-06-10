use anyhow::Result;
use diesel::prelude::*;
use serde::Deserialize;

use crate::app::AppState;
use crate::models::active_model::{CreateArgs, DeleteArgs, UpdateArgs};

use super::handler::Mutation;

#[derive(Debug, Deserialize)]
#[serde(tag = "name", content = "args")]
pub enum ActiveModelMutation {
    #[serde(rename = "createActiveModel")]
    Create(CreateArgs),
    #[serde(rename = "updateActiveModel")]
    Update(UpdateArgs),
    #[serde(rename = "deleteActiveModel")]
    Delete(DeleteArgs),
}

impl ActiveModelMutation {}

impl Mutation for ActiveModelMutation {
    fn process(
        &self,
        state: AppState,
        conn: &mut MysqlConnection,
        user_id: &str,
    ) -> Result<Option<String>> {
        match self {
            ActiveModelMutation::Create(args) => {
                let am = state.service_container.active_model_service.create(
                    conn,
                    args.clone(),
                    user_id,
                )?;
                Ok(Some(am.id))
            }
            ActiveModelMutation::Update(args) => {
                let am = state.service_container.active_model_service.update(
                    conn,
                    args.clone(),
                    user_id,
                )?;
                Ok(Some(am.id))
            }
            ActiveModelMutation::Delete(args) => {
                let am = state
                    .service_container
                    .active_model_service
                    .delete(conn, &args.id, user_id)?;
                Ok(Some(am.id))
            }
        }
    }
}

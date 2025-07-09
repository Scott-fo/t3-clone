use crate::{
    dtos,
    repositories::{Repository, active_model::ActiveModelRepository, chat::ChatRepository},
};
use std::collections::HashMap;

use anyhow::Result;
use diesel::MysqlConnection;

use crate::repositories::message::MessageRepository;

type PatchFn = fn(&mut MysqlConnection, &[&str]) -> Result<HashMap<String, serde_json::Value>>;

macro_rules! make_patch_fn {
    ($repo:path, $resource:path) => {{
        |conn: &mut MysqlConnection, ids: &[&str]| -> Result<_> {
            let map = $repo
                .find_by_ids(conn, ids)?
                .into_iter()
                .map(|record| {
                    let resource: $resource = record.into();
                    serde_json::to_value(&resource).map(|v| (resource.id.clone(), v))
                })
                .collect::<Result<_, _>>()?;
            Ok(map)
        }
    }};
}

pub struct EntityRegistry {
    patch_fns: HashMap<&'static str, PatchFn>,
}

impl EntityRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            patch_fns: HashMap::new(),
        };

        registry.register("chat", make_patch_fn!(ChatRepository, dtos::chat::Chat));
        registry.register(
            "message",
            make_patch_fn!(MessageRepository, dtos::message::Message),
        );
        registry.register(
            "activeModel",
            make_patch_fn!(ActiveModelRepository, dtos::active_model::ActiveModel),
        );

        registry
    }

    pub fn register(&mut self, prefix: &'static str, patch_fn: PatchFn) {
        self.patch_fns.insert(prefix, patch_fn);
    }

    pub fn get_patch_fn(&self, prefix: &str) -> Option<&PatchFn> {
        self.patch_fns.get(prefix)
    }
}

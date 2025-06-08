use crate::repositories::{Repository, chat::ChatRepository};
use std::collections::HashMap;

use anyhow::Result;
use diesel::MysqlConnection;

use crate::repositories::message::MessageRepository;

type PatchFn = Box<
    dyn Fn(&mut MysqlConnection, &[&str]) -> Result<HashMap<String, serde_json::Value>>
        + Send
        + Sync,
>;

pub struct EntityRegistry {
    patch_fns: HashMap<&'static str, PatchFn>,
}

impl EntityRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            patch_fns: HashMap::new(),
        };

        // TODO: Make these dtos so we dont just return the whole thing
        registry.register(
            "chat",
            Box::new(move |conn, ids| {
                let chats = ChatRepository.find_by_ids(conn, ids)?;
                Ok(chats
                    .into_iter()
                    .map(|m| (m.id.clone(), serde_json::to_value(m).unwrap()))
                    .collect())
            }),
        );

        registry.register(
            "message",
            Box::new(move |conn, ids| {
                let messages = MessageRepository.find_by_ids(conn, ids)?;
                Ok(messages
                    .into_iter()
                    .map(|m| (m.id.clone(), serde_json::to_value(m).unwrap()))
                    .collect())
            }),
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

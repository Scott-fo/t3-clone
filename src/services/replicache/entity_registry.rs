use crate::{
    dtos,
    repositories::{Repository, active_model::ActiveModelRepository, chat::ChatRepository},
};
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

        registry.register(
            "chat",
            Box::new(move |conn, ids| {
                let chats = ChatRepository.find_by_ids(conn, ids)?;
                Ok(chats
                    .into_iter()
                    .map(|m| {
                        (
                            m.id.clone(),
                            serde_json::to_value(dtos::chat::Chat::from(m)).unwrap(),
                        )
                    })
                    .collect())
            }),
        );

        registry.register(
            "message",
            Box::new(move |conn, ids| {
                let messages = MessageRepository.find_by_ids(conn, ids)?;
                Ok(messages
                    .into_iter()
                    .map(|m| {
                        (
                            m.id.clone(),
                            serde_json::to_value(dtos::message::Message::from(m)).unwrap(),
                        )
                    })
                    .collect())
            }),
        );

        registry.register(
            "activeModel",
            Box::new(move |conn, ids| {
                let active_models = ActiveModelRepository.find_by_ids(conn, ids)?;
                Ok(active_models
                    .into_iter()
                    .map(|m| {
                        (
                            m.id.clone(),
                            serde_json::to_value(dtos::active_model::ActiveModel::from(m)).unwrap(),
                        )
                    })
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

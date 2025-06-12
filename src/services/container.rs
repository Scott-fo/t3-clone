use std::sync::Arc;

use crate::{
    configuration::Settings,
    repositories::{
        active_model::ActiveModelRepository, chat::ChatRepository, message::MessageRepository,
    },
};

use super::{
    active_model::ActiveModelService, api_key::ApiKeyService, chat::ChatService,
    message::MessageService,
};

#[derive(Debug, Clone)]
pub struct ServiceContainer {
    pub chat_service: ChatService,
    pub message_service: MessageService,
    pub active_model_service: ActiveModelService,
    pub api_key_service: ApiKeyService,
}

impl ServiceContainer {
    pub fn new(config: Arc<Settings>) -> Self {
        Self {
            chat_service: ChatService::new(ChatRepository, MessageRepository),
            message_service: MessageService::new(MessageRepository, ChatRepository),
            active_model_service: ActiveModelService::new(ActiveModelRepository),
            api_key_service: ApiKeyService::new(config.application.secret.clone()),
        }
    }
}

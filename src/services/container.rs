use std::sync::Arc;

use crate::{
    configuration::Settings,
    repositories::{
        active_model::ActiveModelRepository, chat::ChatRepository, message::MessageRepository,
    },
};

use super::{
    active_model::ActiveModelService, api_key::ApiKeyService, chat::ChatService,
    message::MessageService, shared_chat::SharedChatService,
};

#[derive(Debug, Clone)]
pub struct ServiceContainer {
    pub chat_service: ChatService,
    pub message_service: MessageService,
    pub active_model_service: ActiveModelService,
    pub api_key_service: ApiKeyService,
    pub shared_chat_service: SharedChatService,
}

impl ServiceContainer {
    pub fn new(config: Arc<Settings>) -> Self {
        let chat_service = ChatService::new(ChatRepository, MessageRepository);
        let message_service = MessageService::new(MessageRepository, ChatRepository);

        Self {
            chat_service: chat_service.clone(),
            message_service: message_service.clone(),
            active_model_service: ActiveModelService::new(ActiveModelRepository),
            api_key_service: ApiKeyService::new(config.application.secret.clone()),
            shared_chat_service: SharedChatService::new(chat_service, message_service),
        }
    }
}

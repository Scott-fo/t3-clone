use crate::repositories::{
    active_model::ActiveModelRepository, chat::ChatRepository, message::MessageRepository,
};

use super::{active_model::ActiveModelService, chat::ChatService, message::MessageService};

#[derive(Debug, Clone)]
pub struct ServiceContainer {
    pub chat_service: ChatService,
    pub message_service: MessageService,
    pub active_model_service: ActiveModelService,
}

impl ServiceContainer {
    pub fn new() -> Self {
        Self {
            chat_service: ChatService::new(ChatRepository, MessageRepository),
            message_service: MessageService::new(MessageRepository, ChatRepository),
            active_model_service: ActiveModelService::new(ActiveModelRepository),
        }
    }
}

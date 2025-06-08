use crate::repositories::{chat::ChatRepository, message::MessageRepository};

use super::{chat::ChatService, message::MessageService};

#[derive(Debug, Clone)]
pub struct ServiceContainer {
    pub chat_service: ChatService,
    pub message_service: MessageService,
}

impl ServiceContainer {
    pub fn new() -> Self {
        Self {
            chat_service: ChatService::new(ChatRepository, MessageRepository),
            message_service: MessageService::new(MessageRepository, ChatRepository),
        }
    }
}

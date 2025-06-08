use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct SseMessage {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug)]
struct Client {
    user_id: String,
    sender: mpsc::Sender<SseMessage>,
}

#[derive(Debug, Clone, Default)]
pub struct SseManager {
    clients: Arc<RwLock<HashMap<String, Client>>>,
}

impl SseManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn add_client(&self, user_id: String) -> (String, mpsc::Receiver<SseMessage>) {
        let client_id = Uuid::new_v4().to_string();
        let (sender, receiver) = mpsc::channel(32);

        let new_client = Client { user_id, sender };

        self.clients
            .write()
            .await
            .insert(client_id.clone(), new_client);

        info!(client_id, "SSE client added.");
        (client_id, receiver)
    }

    pub async fn remove_client(&self, client_id: &str) {
        if self.clients.write().await.remove(client_id).is_some() {
            info!(client_id, "SSE client removed.");
        }
    }

    pub async fn send_to_user(&self, user_id: &str, msg: SseMessage) {
        let clients = self.clients.read().await;
        let mut sent_count = 0;

        for client in clients.values() {
            if client.user_id == user_id {
                if client.sender.try_send(msg.clone()).is_ok() {
                    sent_count += 1;
                } else {
                    warn!(user_id, "Dropping SSE message; client channel is full.");
                }
            }
        }
        info!(user_id, message_type = %msg.event_type, clients_sent = sent_count, "Sent message to user.");
    }

    pub async fn replicache_poke(&self, user_id: &str) {
        let msg = SseMessage {
            event_type: "replicache-poke".to_string(),
            data: None,
        };
        self.send_to_user(user_id, msg).await;
    }
}

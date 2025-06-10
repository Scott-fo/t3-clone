use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tracing::{debug, info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct SseMessage {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Default)]
pub struct SseManager {
    clients: Arc<RwLock<HashMap<String, HashMap<String, mpsc::Sender<SseMessage>>>>>,
}

impl SseManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn add_client(&self, user_id: String) -> (String, mpsc::Receiver<SseMessage>) {
        let client_id = Uuid::new_v4().to_string();
        let (sender, receiver) = mpsc::channel(4096);

        let mut clients = self.clients.write().await;
        let user_clients = clients.entry(user_id.clone()).or_default();

        user_clients.insert(client_id.clone(), sender);

        info!(client_id, "SSE client added.");
        (client_id, receiver)
    }

    pub async fn remove_client(&self, user_id: &str, client_id: &str) {
        let mut clients = self.clients.write().await;
        let mut user_entry_is_empty = false;

        if let Some(user_clients) = clients.get_mut(user_id) {
            if user_clients.remove(client_id).is_some() {
                info!(%user_id, %client_id, "SSE client removed");
            }

            user_entry_is_empty = user_clients.is_empty();
        }

        if user_entry_is_empty {
            clients.remove(user_id);
            debug!(%user_id, "Removed empty user entry from sse manager");
        }
    }

    pub async fn send_to_user(&self, user_id: &str, msg: SseMessage) {
        let clients = self.clients.read().await;
        let mut sent_count = 0;

        if let Some(user_clients) = clients.get(user_id) {
            for sender in user_clients.values() {
                if sender.try_send(msg.clone()).is_ok() {
                    sent_count += 1;
                } else {
                    warn!(%user_id, "Dropping SSE manager; client channel is full");
                }
            }
        }

        debug!(%user_id, message_type = %msg.event_type, clients_sent = sent_count, "Sent message to user.");
    }

    pub async fn replicache_poke(&self, user_id: &str) {
        let msg = SseMessage {
            event_type: "replicache-poke".to_string(),
            data: None,
        };
        self.send_to_user(user_id, msg).await;
    }
}

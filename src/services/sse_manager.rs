use futures::future::join_all;
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use std::{collections::HashMap, time::Duration};
use tokio::sync::mpsc::error::SendTimeoutError;
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
        let (sender, receiver) = mpsc::channel(128);

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
        let user_clients_clone = {
            let clients = self.clients.read().await;
            clients.get(user_id).cloned()
        };

        let Some(user_clients) = user_clients_clone else {
            debug!(%user_id, "No active SSE clients");
            return;
        };

        let mut send_futures = Vec::new();
        for (client_id, sender) in user_clients {
            let msg_clone = msg.clone();
            let future = async move {
                match sender
                    .send_timeout(msg_clone, Duration::from_millis(100))
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => Err((client_id, e)),
                }
            };
            send_futures.push(future);
        }

        let results = join_all(send_futures).await;

        let mut dead_clients = Vec::new();
        let mut sent_count = 0;

        for result in results {
            match result {
                Ok(_) => sent_count += 1,
                Err((client_id, error)) => match error {
                    SendTimeoutError::Timeout(_) => {
                        warn!(
                            %user_id,
                            %client_id,
                            "SSE send timed out: client channel is full."
                        );
                    }
                    SendTimeoutError::Closed(_) => {
                        warn!(
                            %user_id,
                            %client_id,
                            "SSE channel closed; scheduling client for removal."
                        );
                        dead_clients.push(client_id);
                    }
                },
            }
        }

        debug!(
            %user_id,
            message_type = %msg.event_type,
            %sent_count,
            dead_client_count = dead_clients.len(),
            "Sent message to user."
        );

        if !dead_clients.is_empty() {
            for client_id in dead_clients {
                self.remove_client(user_id, &client_id).await;
            }
        }
    }

    pub async fn replicache_poke(&self, user_id: &str) {
        let msg = SseMessage {
            event_type: "replicache-poke".to_string(),
            data: None,
        };
        self.send_to_user(user_id, msg).await;
    }
}

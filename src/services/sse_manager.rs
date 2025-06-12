use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use tracing::{debug, info};

// make it massive for now. we have all of the backlog available for a chat. so if we lag, we
// should identify the sequence number that we started to fail at, then start sending from the
// backlog.
// probably still needs more thought.
const CHANNEL_CAP: usize = 2000;

#[derive(Debug, Clone)]
struct ChatBacklog {
    msgs: VecDeque<SseMessage>,
}

impl ChatBacklog {
    fn new() -> Self {
        Self {
            msgs: VecDeque::new(),
        }
    }

    fn push(&mut self, msg: SseMessage) {
        self.msgs.push_back(msg);
    }
}

#[derive(Debug, Clone)]
struct UserStream {
    tx: broadcast::Sender<SseMessage>,
    backlogs: HashMap<String, ChatBacklog>,
    open_chats: HashSet<String>,
}

impl UserStream {
    fn new() -> Self {
        let (tx, _) = broadcast::channel(CHANNEL_CAP);
        Self {
            tx,
            backlogs: HashMap::new(),
            open_chats: HashSet::new(),
        }
    }

    fn push(&mut self, msg: SseMessage) {
        let _ = self.tx.send(msg.clone());

        if let Some(chat_id) = extract_chat_id(&msg) {
            let backlog = self
                .backlogs
                .entry(chat_id.to_owned())
                .or_insert_with(|| ChatBacklog::new());

            backlog.push(msg);
        }
    }

    fn mark_chat_open(&mut self, chat_id: &str) {
        self.open_chats.insert(chat_id.to_owned());
    }

    fn mark_chat_closed(&mut self, chat_id: &str) {
        self.open_chats.remove(chat_id);
        if let Some(backlog) = self.backlogs.remove(chat_id) {
            drop(backlog);
        }
    }

    fn all_chats_closed(&self) -> bool {
        self.open_chats.is_empty()
    }

    fn full_backlog_snapshot(&self) -> Vec<SseMessage> {
        self.backlogs
            .values()
            .flat_map(|b| b.msgs.iter().cloned())
            .collect()
    }
}

// make this tagged
#[derive(Debug, Serialize, Clone)]
pub enum EventType {
    #[serde(rename = "chat-stream-chunk")]
    Chunk,
    #[serde(rename = "chat-stream-done")]
    Done,
    #[serde(rename = "chat-stream-error")]
    Err,
    #[serde(rename = "chat-stream-exit")]
    Exit,
    #[serde(rename = "replicache-poke")]
    Replicache,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_string(self).expect("Failed to serialize EventType");
        write!(f, "{}", s.trim_matches('"'))
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SseMessage {
    #[serde(rename = "type")]
    pub event_type: EventType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Default)]
pub struct SseManager {
    inner: Arc<RwLock<HashMap<String, UserStream>>>,
}

impl SseManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn add_client(
        &self,
        user_id: String,
    ) -> (broadcast::Receiver<SseMessage>, Vec<SseMessage>) {
        let mut guard = self.inner.write().await;
        let stream = guard.entry(user_id).or_insert_with(UserStream::new);

        let rx = stream.tx.subscribe();
        let backlog = stream.full_backlog_snapshot();

        info!("SSE client added.");

        (rx, backlog)
    }

    pub async fn send_to_user(&self, user_id: &str, msg: SseMessage) {
        let mut guard = self.inner.write().await;
        if let Some(stream) = guard.get_mut(user_id) {
            update_chat_state(stream, &msg);
            stream.push(msg.clone());
        } else {
            debug!(%user_id, "no active stream; discarding message");
        }
    }

    pub async fn try_gc(&self, user_id: &str) {
        let mut guard = self.inner.write().await;
        if let Some(stream) = guard.get(user_id) {
            if stream.all_chats_closed() && stream.tx.receiver_count() == 0 {
                guard.remove(user_id);
                info!(%user_id, "GC: remove finished stream");
            }
        }
    }

    pub async fn replicache_poke(&self, user_id: &str) {
        let msg = SseMessage {
            event_type: EventType::Replicache,
            data: None,
        };
        self.send_to_user(user_id, msg).await;
    }
}

fn update_chat_state(stream: &mut UserStream, msg: &SseMessage) {
    let chat_id = msg
        .data
        .as_ref()
        .and_then(|v| v.get("chat_id"))
        .and_then(|v| v.as_str());

    if let Some(c_id) = chat_id {
        match msg.event_type {
            EventType::Chunk => stream.mark_chat_open(c_id),
            EventType::Done | EventType::Err => stream.mark_chat_closed(c_id),
            _ => {}
        }
    }
}

fn extract_chat_id(msg: &SseMessage) -> Option<&str> {
    msg.data
        .as_ref()
        .and_then(|v| v.get("chat_id"))
        .and_then(|v| v.as_str())
}

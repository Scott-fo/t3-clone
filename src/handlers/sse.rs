use axum::{
    Extension,
    extract::State,
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures_util::stream::Stream;
use std::{convert::Infallible, pin::Pin, sync::Arc};
use tokio::sync::broadcast;

use crate::{app::AppState, dtos, services::sse_manager::SseManager};

struct SseConnectionGuard {
    manager: Arc<SseManager>,
    user_id: String,
}

impl Drop for SseConnectionGuard {
    fn drop(&mut self) {
        let manager = self.manager.clone();
        let user_id = self.user_id.clone();

        tracing::info!(%user_id, "SSE connection dropped. Cleaning up.");

        tokio::spawn(async move {
            manager.try_gc(&user_id).await;
        });
    }
}

#[axum::debug_handler]
pub async fn sse_handler(
    State(state): State<AppState>,
    Extension(user): Extension<dtos::user::User>,
) -> impl IntoResponse {
    let user_id = user.id;
    let (rx, backlog) = state.sse_manager.add_client(user_id.clone()).await;

    let raw_stream = async_stream::stream! {
        let _guard = SseConnectionGuard {
            manager: state.sse_manager.clone(),
            user_id
        };

        for msg in backlog {
            let json = serde_json::to_string(&msg).unwrap_or_default();
            yield Ok(Event::default().event(msg.event_type.to_string()).data(json));

        }

        let mut rx = rx;
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    let json = serde_json::to_string(&msg).unwrap_or_default();
                    yield Ok(Event::default().event(msg.event_type.to_string()).data(json));
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    tracing::warn!("Message lagged");
                    continue;
                }
                Err(broadcast::error::RecvError::Closed) => {
                    break;
                }
            }
        }
    };

    let stream: Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>> =
        Box::pin(raw_stream);

    Sse::new(stream)
        .keep_alive(KeepAlive::new())
        .into_response()
}

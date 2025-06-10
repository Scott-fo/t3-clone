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
use tokio::time::Duration;

use crate::{app::AppState, dtos, services::sse_manager::SseManager};

struct SseConnectionGuard {
    manager: Arc<SseManager>,
    user_id: String,
    client_id: String,
}

impl Drop for SseConnectionGuard {
    fn drop(&mut self) {
        let manager = self.manager.clone();
        let user_id = self.user_id.clone();
        let client_id = self.client_id.clone();

        tracing::info!(%user_id, %client_id, "SSE connection dropped. Cleaning up.");

        tokio::spawn(async move {
            manager.remove_client(&user_id, &client_id).await;
        });
    }
}

#[axum::debug_handler]
pub async fn sse_handler(
    State(state): State<AppState>,
    Extension(user): Extension<dtos::user::User>,
) -> impl IntoResponse {
    let manager = state.sse_manager;
    let user_id = user.id;

    let (client_id, mut message_rx) = manager.add_client(user_id.clone()).await;

    let stream: Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>> =
        Box::pin(async_stream::stream! {
            let _guard = SseConnectionGuard {
                manager: manager.clone(),
                user_id,
                client_id,
            };

            yield Ok(Event::default().retry(Duration::from_secs(5)));

            while let Some(msg) = message_rx.recv().await {
                let json = serde_json::to_string(&msg).unwrap_or_default();
                yield Ok(Event::default().event(msg.event_type).data(json));
            }
        });

    Sse::new(stream)
        .keep_alive(KeepAlive::new())
        .into_response()
}

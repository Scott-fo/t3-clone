use axum::{
    Extension,
    extract::State,
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures_util::stream::Stream;
use std::{convert::Infallible, pin::Pin};
use tokio::time::{Duration, interval};

use crate::{app::AppState, dtos};

#[axum::debug_handler]
pub async fn sse_handler(
    State(state): State<AppState>,
    Extension(user): Extension<dtos::user::User>,
) -> impl IntoResponse {
    let manager = state.sse_manager;
    let user_id = user.id;

    let (_client_id, mut message_rx) = manager.add_client(user_id).await;

    let stream: Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>> =
        Box::pin(async_stream::stream! {
            yield Ok(Event::default().retry(Duration::from_secs(5)));

            let mut heartbeats = interval(Duration::from_secs(30));

            loop {
                tokio::select! {
                    Some(msg) = message_rx.recv() => {
                        let json = serde_json::to_string(&msg).unwrap_or_default();
                        yield Ok(Event::default().event(msg.event_type).data(json));
                    }
                    _ = heartbeats.tick() => {
                        yield Ok(Event::default().comment("heartbeat"));
                    }
                    else => break,
                }
            }
        });

    Sse::new(stream)
        .keep_alive(KeepAlive::new())
        .into_response()
}

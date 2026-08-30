use std::time::Duration;
use std::{sync::atomic::Ordering};

use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;
use tokio_stream::{Stream, StreamExt};
use tokio_stream::wrappers::BroadcastStream;
use crate::state::{Ping, SharedState};

use axum::response::sse::{Event, KeepAlive, Sse};
use std::convert::Infallible;

#[derive(Serialize)]
struct PingResponse {
    status: &'static str,
    ping_count: i32
}

pub fn get_ping_routes(state:SharedState) -> Router<SharedState> {
    Router::new()
        .route("/test", get(ping_app))
        .route("/listen", get(ping_event_handle))
        .with_state(state)
}

async fn ping_app(State(state): State<SharedState>) -> Json<PingResponse> {
    let count = state.ping_state.ping_count.fetch_add(1, Ordering::Relaxed) + 1;
    let _ = state.ping_state.notify.send(Ping); // just fire the signal
    Json(PingResponse { status: "ok", ping_count: count })
}

async fn ping_event_handle(
    State(state): State<SharedState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let broadcast_rx = state.ping_state.notify.subscribe();
    let stream = BroadcastStream::new(broadcast_rx).map(|result| -> Result<Event, Infallible> {
        let event = match result {
            Ok(_) => Event::default().data("ping").event("ping"),
            Err(e) => Event::default().event("lagged").data(e.to_string()),
        };
        Ok(event)
    });
    Sse::new(stream).keep_alive(
        KeepAlive::new()
        .interval(Duration::from_mins(1))
        .text("keep-alive"))
}

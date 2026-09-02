use std::{collections::{HashMap}, sync::{
    Arc, atomic::AtomicI32,
}};
use axum::{extract::FromRequestParts, http::{StatusCode, request::Parts}};
use tokio::sync::broadcast;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Identity {
    Bib,
    Luc,
}

impl<S> FromRequestParts<S> for Identity
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Identity>()
            .copied()
            .ok_or((StatusCode::UNAUTHORIZED, "missing identity"))
    }
}

pub struct AppState {
    pub ping_state: PingState,
    pub keys: HashMap<String, Identity>
}

#[derive(Clone)]
pub struct Ping;

pub struct PingState {
    pub ping_count: AtomicI32,
    pub notify: broadcast::Sender<Ping>,
}

pub type SharedState = Arc<AppState>;

impl PingState {
    fn new() -> Self {
        let (tx, _rx) = broadcast::channel(16);
        Self {
            ping_count: AtomicI32::new(0),
            notify: tx,
        }
    }
}

impl AppState {
    pub fn new(keys: HashMap<String, Identity>) -> SharedState {
        Arc::new(Self {
            ping_state: PingState::new(),
            keys: keys
        })
    }
}

use std::sync::{
    Arc,
    atomic::{AtomicI32},
};
use tokio::sync::broadcast;

pub struct AppState {
    pub ping_state: PingState,
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
    pub fn new() -> SharedState {
        Arc::new(Self {
            ping_state: PingState::new(),
        })
    }
}

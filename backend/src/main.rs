mod handlers;
mod state;
mod config;
use std::collections::HashMap;

use tracing::{Level, info};

use crate::{config::AppSettings, state::{AppState, Identity}};

fn load_config() -> AppSettings {
    let config = std::fs::read_to_string("config.toml").expect("Failed to read file");
    toml::from_str(&config).expect("Error parsing config")
}


fn load_auth_keys() -> HashMap<String, Identity> {
    let key1 = std::env::var("BIB_KEY").expect("API1_KEY must be set (check .env)");
    let key2 = std::env::var("LUC_KEY").expect("API2_KEY must be set (check .env)");

    let mut map = HashMap::new();
    map.insert(key1, Identity::Bib);
    map.insert(key2, Identity::Luc);
    map
}


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let config = load_config();
    let keys = load_auth_keys();

    info!("Config loaded succesfully {}", config.server.to_addr());
    let state = AppState::new(keys);
    let app = handlers::app_router(state);

    let listener = tokio::net::TcpListener::bind(&config.server.to_addr()).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

use axum::{Json, Router, response::IntoResponse, routing::get};
use serde_json::json;

mod handlers;
mod state;

use crate::state::AppState;

const IP_ADDR: &str = "0.0.0.0";
const PORT: i32 = 7878;


#[tokio::main]
async fn main() {
    // build our application with a single route
    let state = AppState::new();
    let app = handlers::app_router(state);
    let route = format!("{IP_ADDR}:{PORT}");
    let listener = tokio::net::TcpListener::bind(&route).await.unwrap();
    println!("Server started successfully at {route}");
    axum::serve(listener, app).await.unwrap();
}

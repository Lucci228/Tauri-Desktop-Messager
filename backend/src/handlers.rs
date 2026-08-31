use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::{handlers::ping_handler::get_ping_routes, state::SharedState};

pub mod ping_handler;

// use crate::AppState;

// Function to create the main application router TO DO add app state
pub fn app_router(state: SharedState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:1420".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // Define the root route
        .route("/", get(root))
        .route("/beb", get(special_test))
        //TO DO ADD NESTED PATHS HERE
        // Define a fallback handler for 404 errors
        .nest("/ping", get_ping_routes(state.clone()))
        .fallback(handler_404)
        .layer(cors)
        .with_state(state)
        // Attach the application state to the router
}

// Handler for the root route
async fn root() -> &'static str {
    "Server is running!"
}

async fn special_test() -> &'static str {
    "TE IUBESC MUWA MUWAH"
}

// Handler for 404 Not Found errors
async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::{handlers::ping_handler::get_ping_routes, state::SharedState};

pub mod ping_handler;

// use crate::AppState;

// Function to create the main application router TO DO add app state
pub fn app_router(state: SharedState) -> Router {
    Router::new()
        // Define the root route
        .route("/", get(root))
        //TO DO ADD NESTED PATHS HERE
        // Define a fallback handler for 404 errors
        .nest("/ping", get_ping_routes(state.clone()))
        .fallback(handler_404)
        .with_state(state)
        // Attach the application state to the router
}

// Handler for the root route
async fn root() -> &'static str {
    "Server is running!"
}

// Handler for 404 Not Found errors
async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}

use axum::{
    Router, extract::{Request, State}, http::{self, HeaderMap, StatusCode}, middleware::{self, Next}, response::{IntoResponse, Response}, routing::{get, post},
};
use tower::ServiceBuilder;
use tower_http::{cors::{Any, CorsLayer}, trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer}};
use tracing::Level;
use tracing_subscriber::fmt::layer;

use crate::{config::AppSettings, handlers::ping_handler::get_ping_routes, state::{Identity, SharedState}};

pub mod ping_handler;


// Function to create the main application router TO DO add app state
pub fn app_router(state: SharedState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:1420".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO));

    let middleware_layer = ServiceBuilder::new()
        .layer(trace_layer)
        .layer(middleware::from_fn_with_state(state.clone(), auth_service))
        .layer(cors);


    Router::new()
        .route("/", get(root))
        .route("/ident", get(test_identity))
        .route("/beb", get(special_test))
        .nest("/ping", get_ping_routes(state.clone()))
        .fallback(handler_404)
        .layer(middleware_layer)
        .with_state(state)
}

// Handler for the root route
async fn root() -> &'static str {
    "Server is running!"
}

async fn special_test() -> &'static str {
    "TE IUBESC MUWA MUWAH"
}

async fn test_identity(identity: Identity) -> &'static str {
    match identity {
        Identity::Bib => "Salut bibanu",
        Identity::Luc => "Salut bert"
    }
}

// Handler for 404 Not Found errors
async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}

async fn auth_service(State(state): State<SharedState>, headers : HeaderMap, mut request : Request, next: Next) -> Result<Response, StatusCode> {
    let token = headers
        .get(http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(http::StatusCode::UNAUTHORIZED)?;

    match state.keys.get(token) {
        Some(identity) => {
            request.extensions_mut().insert(identity.clone());
            Ok(next.run(request).await)
        },
        None => Err(StatusCode::UNAUTHORIZED)
    }

}

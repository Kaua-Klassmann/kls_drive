use axum::{Router, routing::get};

pub fn configure_routes() -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}

use axum::{Router, routing::get};

use crate::state::AppState;

pub fn configure_routes() -> Router<AppState> {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}

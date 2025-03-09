use axum::{Router, routing::get};
use document::document_routes;
use user::user_routes;

use crate::state::AppState;

mod document;
mod user;

pub fn configure_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/user", user_routes())
        .nest("/document", document_routes())
}

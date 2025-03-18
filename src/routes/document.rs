use axum::{Router, routing::post};

use crate::{handlers, state::AppState};

pub fn document_routes() -> Router<AppState> {
    Router::new().route("/upload", post(handlers::document::upload_document))
}

use axum::{
    Router,
    routing::{get, post},
};

use crate::{handlers, state::AppState};

pub fn document_routes() -> Router<AppState> {
    Router::new()
        .route("/upload", post(handlers::document::upload_document))
        .route("/all", get(handlers::document::view_all_documents_per_page))
        .route("/delete/{id}", get(handlers::document::delete))
}

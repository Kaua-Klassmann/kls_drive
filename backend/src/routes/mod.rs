use axum::{Router, routing::get};
use user::user_routes;

use crate::state::AppState;

mod user;

pub fn configure_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/user", user_routes())
}

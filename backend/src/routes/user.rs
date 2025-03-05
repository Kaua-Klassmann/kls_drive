use axum::{
    Router,
    routing::{get, post},
};

use crate::{handlers, state::AppState};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::user::register_user))
        .route(
            "/activate/{activate_code}",
            get(handlers::user::activate_user),
        )
}

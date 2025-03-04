use std::sync::Arc;

use argon2::Argon2;
use axum::Router;

use crate::{
    connections::{
        database::get_db_connections, email::get_email_mailer, redis::get_redis_connection,
    },
    middleware::cors::get_cors,
    routes::configure_routes,
    state::AppState,
};

pub async fn create_app() -> Router {
    let db_conn = get_db_connections().await;
    let redis_conn = Arc::new(get_redis_connection().await);
    let email_mailer = Arc::new(get_email_mailer());
    let argon2 = Arc::new(Argon2::default());

    let state = AppState {
        db_conn,
        redis_conn,
        email_mailer,
        argon2,
    };

    let app = configure_routes().layer(get_cors()).with_state(state);

    app
}

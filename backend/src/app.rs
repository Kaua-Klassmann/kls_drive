use axum::Router;

use crate::{
    database::database::get_db_connections, middleware::cors::get_cors, routes::configure_routes,
    state::AppState,
};

pub async fn create_app() -> Router {
    let db_conn = get_db_connections().await;

    let state = AppState { db_conn };

    let app = configure_routes().layer(get_cors()).with_state(state);

    app
}

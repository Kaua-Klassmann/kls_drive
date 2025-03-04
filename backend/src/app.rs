use axum::Router;

use crate::{middleware::cors::get_cors, routes::configure_routes};

pub fn create_app() -> Router {
    let app = configure_routes().layer(get_cors());

    app
}

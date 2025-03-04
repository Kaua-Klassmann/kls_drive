use axum::Router;

use crate::routes::configure_routes;

pub fn create_app() -> Router {
    let app = configure_routes();

    app
}

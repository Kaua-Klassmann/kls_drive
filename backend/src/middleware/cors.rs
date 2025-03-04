use tower_http::cors::CorsLayer;

use crate::config::cors::get_cors_config;

pub fn get_cors() -> CorsLayer {
    let cors_config = get_cors_config();

    CorsLayer::new()
        .allow_origin(cors_config.origin)
        .allow_methods(cors_config.methods)
        .allow_headers(cors_config.headers)
}

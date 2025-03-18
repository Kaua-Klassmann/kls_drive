use std::env;

use axum::http::{HeaderValue, Method};
use tower_http::cors::Any;

pub struct CorsConfig {
    pub origin: HeaderValue,
    pub methods: Vec<Method>,
    pub headers: Any,
}

pub fn get_cors_config() -> CorsConfig {
    let origin = env::var("CORS_ORIGIN")
        .expect("CORS_ORIGIN not found at .env file")
        .parse::<HeaderValue>()
        .unwrap();

    let methods = vec![Method::GET, Method::POST, Method::PUT, Method::DELETE];

    let headers = Any;

    CorsConfig {
        origin,
        methods,
        headers,
    }
}

use std::env;

use app::create_app;
use dotenvy::dotenv;
use tokio::net::TcpListener;

mod app;
mod config;
mod middleware;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("APP_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind port");

    let app = create_app();

    println!("Server listening on port {}", port);

    axum::serve(listener, app).await.unwrap();
}

use app::create_app;
use dotenvy::dotenv;
use tokio::net::TcpListener;

mod app;
mod config;
mod connections;
mod handlers;
mod jwt;
mod middleware;
mod routes;
mod services;
mod state;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = config::app::get_app_config().port;

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind port");

    let app = create_app().await;

    println!("Server listening on port {}", port);

    axum::serve(listener, app).await.unwrap();
}

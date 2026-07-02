mod controllers;
mod models;

use axum::{Router, routing::get, serve};
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").expect("PORT environment variable must be set");
    let app = Router::new()
        .route("/", get(controllers::index))
        .route("/status", get(controllers::status))
        .fallback(controllers::not_found);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.expect("failed to bind to network port");
    serve(listener, app).await.expect("failed to start the axum server");
}

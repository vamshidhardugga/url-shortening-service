use std::env;

use axum::{Json, Router, http::StatusCode, routing::get, serve};
use serde::Serialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // TODO: Implement automatic loading of the .env file, or a declarative
    // approach to read from specific environment files like .env.local or .env.development.
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let app = Router::new()
        .route("/", get(home))
        .route("/status", get(status))
        .fallback(not_found);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("failed to bind to port 3000");
    serve(listener, app)
        .await
        .expect("failed to start the axum server");
}

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

async fn home() -> Json<Response> {
    Json(Response {
        message: "URL shortening service is active",
    })
}

async fn status() -> Json<Response> {
    Json(Response {
        message: "Server is up and running",
    })
}

async fn not_found() -> (StatusCode, Json<Response>) {
    (
        StatusCode::NOT_FOUND,
        Json(Response {
            message: "Requested resource not found",
        }),
    )
}

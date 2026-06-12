use axum::{Json, http::StatusCode};

use crate::models::Response;

pub async fn index() -> Json<Response> {
    Json(Response { message: "URL shortening service is active".into() })
}

pub async fn status() -> Json<Response> {
    Json(Response { message: "Server is up and running".into() })
}

pub async fn not_found() -> (StatusCode, Json<Response>) {
    (StatusCode::NOT_FOUND, Json(Response { message: "Requested resource not found".into() }))
}

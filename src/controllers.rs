use crate::models::GenericResponse;
use axum::{Json, http::StatusCode};

pub async fn index() -> (StatusCode, Json<GenericResponse>) {
    (StatusCode::OK, Json(GenericResponse { message: "URL shortening service is active".into() }))
}

pub async fn status() -> (StatusCode, Json<GenericResponse>) {
    (StatusCode::OK, Json(GenericResponse { message: "Server is up and running".into() }))
}

pub async fn not_found() -> (StatusCode, Json<GenericResponse>) {
    (StatusCode::NOT_FOUND, Json(GenericResponse { message: "Requested resource not found".into() }))
}

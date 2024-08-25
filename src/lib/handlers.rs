// src/lib/handlers.rs

// dependencies
use axum::{http::StatusCode, response::IntoResponse};

// health check handler
pub async fn health() -> impl IntoResponse {
    StatusCode::OK
}

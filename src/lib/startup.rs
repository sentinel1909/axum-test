// src/lib/startup.rs

// dependencies
use crate::handlers::health;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

// struct to represent the application type
pub struct Application(pub Router);

// methods for the Application type
impl Application {
    pub fn build() -> Self {
        let router = Router::new()
            .route("/health", get(health))
            .nest_service("/", ServeDir::new("static"));

        Self(router)
    }
}

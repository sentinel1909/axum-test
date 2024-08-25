// src/main.rs

// dependencies
use axum_test_lib::Application;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let api = Application::build();

    Ok(api.0.into())
}

use crate::utils::shared_state::new_shared_state;
use axum::Router;
use dotenv::dotenv;

mod controllers;
mod services;
mod stores;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or("8000".to_string());
    let port = match port.parse::<u32>() {
        Ok(port) => port,
        Err(_err) => {
            panic!("Invalid port passed")
        }
    };

    let shared_state = new_shared_state().await;

    let app = Router::new().nest(
        "/example",
        controllers::example_controller::get_router(shared_state.clone()),
    );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("Listening on port {}", port);
    axum::serve(listener, app).await.unwrap();
}

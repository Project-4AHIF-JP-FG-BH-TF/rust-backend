use axum::Router;
use crate::utils::shared_state::{new_shared_state};

mod controllers;
mod services;
mod stores;
mod utils;

#[tokio::main]
async fn main() {
    let shared_state = new_shared_state().await;

    let app = Router::new().nest("/example", controllers::example_controller::get_router(shared_state.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on port 8000");
    axum::serve(listener, app).await.unwrap();
}

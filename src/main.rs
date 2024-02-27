use crate::utils::shared_state::new_shared_state;
use axum::http::Method;
use axum::Router;
use dotenvy::dotenv;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod controllers;
mod services;
mod stores;
mod utils;
mod schema;
mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_level(true))
        .with(LevelFilter::INFO)
        .init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let port = std::env::var("PORT").unwrap_or("8000".to_string());
    let port = port.parse::<u32>().expect("Invalid port passed");

    let shared_state = new_shared_state().await;
    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .nest(
                    "/example",
                    controllers::example_controller::get_router(shared_state.clone()),
                )
                .nest(
                    "/file",
                    controllers::file_controller::get_router(shared_state.clone()),
                ),
        )
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(LatencyUnit::Micros),
                    ),
            ),
        )
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("Listening on port {}", port);
    axum::serve(listener, app).await.unwrap();
}

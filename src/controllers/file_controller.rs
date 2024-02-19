use crate::services;
use crate::utils::shared_state::SharedState;
use axum::extract::{DefaultBodyLimit, Multipart};
use axum::http::StatusCode;
use axum::routing::post;
use axum::Router;

pub fn get_router(shared_state: SharedState) -> Router {
    Router::new()
        .route("/upload", post(upload))
        .with_state(shared_state)
        .layer(DefaultBodyLimit::disable())
}

async fn upload(multipart: Multipart) -> (StatusCode, String) {
    match services::file_service::extract_zip(multipart).await {
        Ok(_) => (StatusCode::OK, String::from("Was successfully uploaded!")),
        Err(err) => err,
    }
}

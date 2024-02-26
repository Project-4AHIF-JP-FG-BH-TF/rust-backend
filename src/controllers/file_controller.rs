use crate::services;
use crate::utils::shared_state::SharedState;
use axum::extract::{DefaultBodyLimit, Multipart, Path};
use axum::http::StatusCode;
use axum::routing::post;
use axum::Router;

pub fn get_router(shared_state: SharedState) -> Router {
    Router::new()
        .route("/upload/:session_id", post(upload))
        .with_state(shared_state)
        .layer(DefaultBodyLimit::disable())
}

async fn upload(Path(sessions_id): Path<String>, multipart: Multipart) -> (StatusCode, String) {
    match services::file_service::extract_zip(multipart, sessions_id).await {
        Ok(_) => (StatusCode::OK, String::from("Was successfully uploaded!")),
        Err(err) => err,
    }
}

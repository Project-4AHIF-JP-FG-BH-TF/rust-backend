use crate::utils::shared_state::SharedState;
use crate::{services};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Extension, Router};
use std::collections::HashMap;

pub fn get_router(shared_state: SharedState) -> Router {
    Router::new()
        .layer(Extension(shared_state))
        .route("/greet", get(greet))
}

async fn greet(
    Extension(state): Extension<SharedState>,
    Query(params): Query<HashMap<String, String>>,
) -> (StatusCode, String) {
    let name = params.get("name").cloned();

    match services::example_service::greet(name).await {
        Ok(message) => (StatusCode::OK, message),
        Err(error) => (error.code, error.message),
    }
}

use crate::utils::shared_state::{AppState, SharedState};
use crate::{services};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Router};
use std::collections::HashMap;
use std::sync::Arc;

pub fn get_router(shared_state: SharedState) -> Router {
    Router::new()
        .route("/greet", get(greet))
        .with_state(shared_state)
}

async fn greet(
    State(state): State<SharedState>,
    Query(params): Query<HashMap<String, String>>,
) -> (StatusCode, String) {
    let name = params.get("name").cloned();

    match services::example_service::greet(name).await {
        Ok(message) => (StatusCode::OK, message),
        Err(error) => (error.code, error.message),
    }
}

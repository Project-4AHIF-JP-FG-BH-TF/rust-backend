use crate::utils::backend_error::BackendError;
use crate::{stores, utils};
use axum::http::StatusCode;

pub async fn greet(name: Option<String>) -> Result<String, BackendError> {
    if let Some(name) = name.clone() {
        if name.to_lowercase() == "benedikt" {
            return Err(BackendError::new(
                StatusCode::BAD_REQUEST,
                "I hate Benedikt".to_string(),
            ));
        }
    }

    stores::example_store::do_something().await;

    Ok(utils::example::greet(name))
}

use axum::http::StatusCode;

pub struct BackendError {
    pub code: StatusCode,
    pub message: String,
}

impl BackendError {
    pub fn new<C: Into<StatusCode>, M: Into<String>>(code: C, message: M) -> Self {
        fn new(code: StatusCode, message: String) -> BackendError {
            BackendError { code, message }
        }
        new(code.into(), message.into())
    }
}

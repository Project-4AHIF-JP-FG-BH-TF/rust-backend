use std::sync::Arc;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub type SharedState = Arc<AppState>;

pub async fn new_shared_state() -> SharedState {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:admin@localhost/loggaroo")
        .await
        .expect("Failed to connect to db");

    Arc::new(AppState { pool })
}

pub struct AppState {
    pub(crate) pool: Pool<Postgres>
}
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env::var;
use std::sync::Arc;

pub type SharedState = Arc<AppState>;

pub async fn new_shared_state() -> SharedState {
    let database_url = match var("DATABASE_URL") {
        Ok(database_url) => database_url,
        Err(_) => panic!("No DatabaseURL provided!"),
    };

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to db");

    Arc::new(AppState { pool })
}

pub struct AppState {
    pub(crate) pool: Pool<Postgres>,
}

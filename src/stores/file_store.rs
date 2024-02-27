use crate::types::tmp_message::TmpMessage;
use crate::utils::shared_state::SharedState;
use sqlx::types::Uuid;
use sqlx::{Executor, Pool, Postgres};
use std::str::FromStr;

pub async fn store_messages(
    pool: &Pool<Postgres>,
    session_id: String,
    file_name: String,
    messages: &[TmpMessage],
) {
    for (index, message) in messages.iter().enumerate() {
        pool.execute(sqlx::query!("INSERT INTO loggaroo.log_entry(session_id, file_name, entry_nr, creation_date, classification, service_ip, user_id, user_session_id, java_class, content) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10);",
                Uuid::from_str(&session_id).unwrap(),
                file_name,
                index as i32,
                message.creation_date,
                message.classification.to_lowercase(),
                message.user_id,
                message.service_ip,
                message
                    .user_session_id,
                message.java_class,
                message.content)).await.expect("Failed to upload to db!");
    }
}

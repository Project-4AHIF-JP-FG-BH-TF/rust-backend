use crate::types::tmp_message::TmpMessage;
use sqlx::types::Uuid;
use sqlx::{query, QueryBuilder};
use std::str::FromStr;

pub fn store_messages(session_id: String, file_name: String, messages: &[TmpMessage]) {
    messages
        .iter()
        .enumerate()
        .for_each(|(index, message)| {
            query!("INSERT INTO loggaroo.log_entry(session_id, file_name, entry_nr, creation_date, classification, service_ip, user_id, user_session_id, java_class, content) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10);",
                Uuid::from_str(&session_id).unwrap(),
                file_name,
                index,
                message.creation_date,
                message.classification,
                message.user_id.as_ref(),
                message.service_ip.as_ref(),
                message
                    .user_session_id
                    .as_ref()
                    .unwrap_or(&"null".to_string()),
                message.java_class,
                message.content);

            format!(
                "({session_id}, {file_name}, {index}, {}, {}, {}, {}, {}, {}, {})\n",
                message.creation_date,
                message.classification,
                message.user_id.as_ref().unwrap_or(&"null".to_string()),
                message.service_ip.as_ref().unwrap_or(&"null".to_string()),
                message
                    .user_session_id
                    .as_ref()
                    .unwrap_or(&"null".to_string()),
                message.java_class,
                message.content
            );
        });
}

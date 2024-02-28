use crate::models::loggaroo::{File, LogMessage};
use crate::schema;
use deadpool_diesel::postgres::Pool;
use diesel::RunQueryDsl;

pub async fn store_messages(messages: Vec<LogMessage>, pool: &Pool) {
    pool.get()
        .await
        .unwrap()
        .interact(move |connection| {
            for messages in messages.chunks(5000) {
                diesel::insert_into(schema::loggaroo::log_entry::table)
                    .values(messages)
                    .execute(connection)
                    .unwrap();
            }
        })
        .await
        .unwrap();
}

pub async fn store_files(files: Vec<File>, pool: &Pool) {
    pool.get()
        .await
        .unwrap()
        .interact(|connection| {
            diesel::insert_into(schema::loggaroo::file::table)
                .values(files)
                .execute(connection)
                .unwrap();
        })
        .await
        .unwrap();
}

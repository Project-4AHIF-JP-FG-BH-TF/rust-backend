// @generated automatically by Diesel CLI.

pub mod loggaroo {
    diesel::table! {
        loggaroo.file (session_id, file_name) {
            session_id -> Uuid,
            file_name -> Varchar,
            hash -> Varchar,
            chunk_count -> Int4,
            uploaded_chunk_count -> Int4,
        }
    }

    diesel::table! {
        loggaroo.log_entry (session_id, file_name, entry_nr) {
            session_id -> Uuid,
            file_name -> Varchar,
            entry_nr -> Int4,
            creation_date -> Timestamp,
            classification -> Varchar,
            service_ip -> Nullable<Varchar>,
            user_id -> Nullable<Varchar>,
            user_session_id -> Nullable<Varchar>,
            java_class -> Varchar,
            content -> Varchar,
            sql_raw -> Nullable<Varchar>,
            sql_data -> Nullable<Varchar>,
        }
    }

    diesel::table! {
        loggaroo.session (session_id) {
            session_id -> Uuid,
            last_refresh -> Timestamp,
        }
    }

    diesel::joinable!(file -> session (session_id));

    diesel::allow_tables_to_appear_in_same_query!(file, log_entry, session,);
}

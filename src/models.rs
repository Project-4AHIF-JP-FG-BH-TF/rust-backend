pub mod loggaroo {
    use diesel::{Insertable, Queryable, Selectable};
    use time::PrimitiveDateTime;
    use uuid::Uuid;

    #[derive(Queryable, Selectable, Insertable, Debug)]
    #[diesel(table_name = crate::schema::loggaroo::log_entry)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct LogMessage {
        pub session_id: Uuid,
        pub file_name: String,
        pub entry_nr: i32,
        pub creation_date: PrimitiveDateTime,
        pub classification: String,
        pub service_ip: Option<String>,
        pub user_id: Option<String>,
        pub user_session_id: Option<String>,
        pub java_class: String,
        pub content: String,
        pub sql_raw: Option<String>,
        pub sql_data: Option<String>,
    }

    #[derive(Queryable, Selectable, Insertable, Debug)]
    #[diesel(table_name = crate::schema::loggaroo::file)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct File {
        pub session_id: Uuid,
        pub file_name: String,
        pub hash: String,
        pub chunk_count: i32,
        pub uploaded_chunk_count: i32,
    }
}

use sqlx::types::time::Date;

#[derive(Debug)]
pub struct TmpMessage {
    pub creation_date: Date,
    pub classification: String,
    pub service_ip: Option<String>,
    pub user_id: Option<String>,
    pub user_session_id: Option<String>,
    pub java_class: String,
    pub content: String,
}

impl TmpMessage {
    pub fn new(creation_date: Date, classification: String, service_ip: Option<String>, user_id: Option<String>, user_session_id: Option<String>, java_class: String, content: String) -> Self {
        Self { creation_date, classification, service_ip, user_id, user_session_id, java_class, content }
    }
}
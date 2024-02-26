use sqlx::types::time::Date;

#[derive(Debug)]
pub struct TmpMessage {
    creation_date: Date,
    classification: String,
    service_ip: Option<String>,
    user_id: Option<String>,
    user_session_id: Option<String>,
    java_class: String,
    content: String,
}

impl TmpMessage {
    pub fn new(creation_date: Date, classification: String, service_ip: Option<String>, user_id: Option<String>, user_session_id: Option<String>, java_class: String, content: String) -> Self {
        Self { creation_date, classification, service_ip, user_id, user_session_id, java_class, content }
    }
}
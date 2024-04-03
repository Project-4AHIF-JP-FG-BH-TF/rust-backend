use deadpool_diesel::postgres::Pool;
use deadpool_diesel::Manager;
use regex::Regex;
use std::env::var;
use std::sync::Arc;

pub type SharedState = Arc<AppState>;

pub async fn new_shared_state() -> SharedState {
    let database_url = var("DATABASE_URL").expect("No DatabaseURL provided!");

    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);

    let pool = Pool::builder(manager)
        .build()
        .expect("Failed to connect to db!");

    Arc::new(AppState {
        pool: Arc::new(pool),
        message_regex: message_regex(),
    })
}

fn message_regex() -> Regex {
    let date_part = r"\d+-\d{2}-\d{2} \d{2}:\d{2}:\d{2},\d{3}";
    let classification_part = r"[a-zA-Z]+";
    let ip_part = r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\s*";
    let user_id_part = r"\d+\s*";
    let session_id_part = r"[A-Z0-9]+ \d+\s*";
    let java_class_part = r"\S*\s*";
    let content_part = r".*";

    let regex = format!(
        r"^({date_part})\s*({classification_part})\s*\[({ip_part})?\s*]\s*\[({user_id_part})?\s*]\s*\[({session_id_part})?\s*]\s*\[({java_class_part})\s*]\s*({content_part})$"
    );
    println!("{}", regex);

    Regex::new(&regex).unwrap()
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<Pool>,
    pub message_regex: Regex,
}

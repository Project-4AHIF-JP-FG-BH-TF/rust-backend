/// Returns "Hello {name}!". If there is no name passed "Hello there!"
pub fn greet(name: Option<String>) -> String {
    let name = match name {
        None => "there".to_string(),
        Some(name) => name,
    };

    format!("Hello {}", name)
}
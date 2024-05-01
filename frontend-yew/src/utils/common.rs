use std::str::FromStr;
use chrono::{DateTime, Utc, format::strftime::StrftimeItems};
use serde_json::Value;

pub fn format_date(date: &Value, format: &str) -> Result<String, Box<dyn std::error::Error>> {

    let datetime = DateTime::<Utc>::from_str(date.as_str().unwrap())?;
    
    // 使用 chrono 的格式化功能
    let format = StrftimeItems::new(format);
    let formatted = datetime.format_with_items(format).to_string();
    Ok(formatted)
}

pub fn is_logged_in() -> bool {
    super::token::get_pair_value("jwt").is_some()
}

pub fn create_gravatar_url(email: &str, size: i64) -> String {
    let trimmed_email = email.trim().to_lowercase();
    let hash = md5::compute(trimmed_email.as_bytes());
    format!("https://www.gravatar.com/avatar/{:x}?d=identicon&s={}", hash, size)
}
use chrono::{format::strftime::StrftimeItems, DateTime, Utc};
use serde_json::Value;
use wasm_bindgen::JsValue;
use std::str::FromStr;

pub fn format_date(date: &Value, format: &str) -> Result<String, Box<dyn std::error::Error>> {
    let datetime = DateTime::<Utc>::from_str(date.as_str().unwrap())?;

    // 使用 chrono 的格式化功能
    let format = StrftimeItems::new(format);
    let formatted = datetime.format_with_items(format).to_string();
    Ok(formatted)
}

pub fn is_logged_in() -> bool {
    super::storage::get_pair_value("jwt").is_some()
}

pub fn create_gravatar_url(email: &str, size: i64) -> String {
    let trimmed_email = email.trim().to_lowercase();
    let hash = md5::compute(trimmed_email.as_bytes());
    // format!("https://www.gravatar.com/avatar/{:x}?d=identicon&s={}", hash, size)
    format!(
        "https://dn-qiniu-avatar.qbox.me/avatar/{:x}?d=identicon&s={}",
        hash, size
    )
}

// 把 summary 变得更简短

pub fn shorter_string(summary: &str, length: usize) -> String {
    // summary 为 utf-8 中文文本
    let mut result = String::new();
    let mut current_length = 0;
    for c in summary.chars() {
        let char_length = c.len_utf8();
        if current_length + char_length > length {
            break;
        }
        result.push(c);
        current_length += char_length;
    }
    if result.len() < summary.len() {
        result.push_str("...");
    } else {
        web_sys::console::log_2(&JsValue::from_str(&result), &JsValue::from_f64(current_length as f64));
    }
    result
}
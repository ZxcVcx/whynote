use std::str::FromStr;
use chrono::{DateTime, Utc, format::strftime::StrftimeItems};
use serde_json::Value;



pub fn format_date(date: &Value) -> Result<String, Box<dyn std::error::Error>> {

    let datetime = DateTime::<Utc>::from_str(date.as_str().unwrap())?;
    
    // 使用 chrono 的格式化功能
    let format = StrftimeItems::new("%b %d");
    let formatted = datetime.format_with_items(format).to_string();
    Ok(formatted)
}

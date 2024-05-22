// use serde_json::{json, Value};
use super::articles::fetch_articles_data;
use super::use_query::FetchError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchData {
    pub id: String,
    pub subject: String,
    pub summary: String,
    pub content: String,
    pub slug: String,
}

pub async fn fetch_articles_data_as_vec() -> Result<Vec<SearchData>, FetchError> {
    let articles = fetch_articles_data()
        .await
        .unwrap()
        .get("articles")
        .unwrap()
        .clone();
    let articles: Vec<SearchData> = serde_json::from_value(articles).unwrap();
    Ok(articles)
}

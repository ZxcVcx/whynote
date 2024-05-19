// use serde_json::{json, Value};
use super::use_query::FetchError;
use serde::{Deserialize, Serialize};
use super::articles::fetch_articles_data;

// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "./graphql/schema.graphql",
//     query_path = "./graphql/articles.graphql",
//     response_derives = "Debug, Clone"
// )]
// struct ArticlesData;

// // https://blog.lanesawyer.dev/32266/yew-hooks-with-graphql




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchData {
    pub id: String,
    pub subject: String,
    pub summary: String,
    pub content: String,
    pub slug: String,
}

// pub async fn fetch_search_data() -> Result<Value, FetchError> {
//     let articles = fetch_articles_data().await.unwrap().get("articles").unwrap().clone();
//     let articles: Vec<SearchData> = serde_json::from_value(articles).unwrap();
//     let search_data = json!({
//         "searchData": articles,
//     });
//     Ok(search_data) 
// }

pub async fn fetch_articles_data_as_vec() -> Result<Vec<SearchData>, FetchError> {
    let articles = fetch_articles_data().await.unwrap().get("articles").unwrap().clone();
    let articles: Vec<SearchData> = serde_json::from_value(articles).unwrap();
    Ok(articles) 
}

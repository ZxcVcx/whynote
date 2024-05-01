use graphql_client::GraphQLQuery;
use serde_json::{json, Value};

use super::use_query::FetchError;

type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/article.graphql",
    response_derives = "Debug, Clone"
)]
struct ArticleData;

// https://blog.lanesawyer.dev/32266/yew-hooks-with-graphql

async fn query(username: String, slug: String) -> Value {
    let build_query = ArticleData::build_query(article_data::Variables { username, slug });
    let query = json!(build_query);
    query
}

pub async fn fetch_article_data_by_username_and_slug(username: String, slug: String) -> Result<Value, FetchError> {
    let query = query(username, slug).await;
    let data = super::use_query::fetch_gql_data(query).await;
    data
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/article.graphql",
    response_derives = "Debug, Clone"
)]
struct ArticleDataBySlug;

pub async fn fetch_article_data_by_slug(slug: String) -> Result<Value, FetchError> {
    let veriables = article_data_by_slug::Variables { slug };
    let query_body = ArticleDataBySlug::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;

    Ok(data)
}
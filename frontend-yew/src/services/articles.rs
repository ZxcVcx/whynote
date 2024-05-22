use graphql_client::GraphQLQuery;
use serde_json::{json, Value};

use super::use_query::FetchError;

type DateTime = String;
type ObjectId = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/articles.graphql",
    response_derives = "Debug, Clone"
)]
struct ArticlesData;

// https://blog.lanesawyer.dev/32266/yew-hooks-with-graphql

async fn articles_query() -> Value {
    let veriables = articles_data::Variables {};
    let query_body = ArticlesData::build_query(veriables);
    let query = json!(query_body);
    query
}

pub async fn fetch_articles_data() -> Result<Value, FetchError> {
    let query = articles_query().await;
    let data = super::use_query::fetch_gql_data(query).await;
    data
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/articles.graphql",
    response_derives = "Debug, Clone"
)]
struct CraftsData;

async fn crafts_query() -> Value {
    let veriables = crafts_data::Variables {};
    let query_body = CraftsData::build_query(veriables);
    let query = json!(query_body);
    query
}

pub async fn fetch_crafts_data() -> Result<Value, FetchError> {
    let query = crafts_query().await;
    let data = super::use_query::fetch_gql_data(query).await;
    data
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/home.graphql",
    response_derives = "Debug, Clone"
)]
struct HomeData;

async fn home_query() -> Value {
    let veriables = home_data::Variables {};
    let query_body = HomeData::build_query(veriables);
    let query = json!(query_body);

    query
}

pub async fn fetch_home_data() -> Result<Value, FetchError> {
    let query = home_query().await;
    let data = super::use_query::fetch_gql_data(query).await;
    data
}

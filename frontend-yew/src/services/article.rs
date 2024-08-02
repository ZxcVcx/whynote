use graphql_client::GraphQLQuery;
use serde_json::{json, Value};

// use crate::components::main_container::article::ArticleProps::article;

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

pub async fn fetch_article_data_by_username_and_slug(
    username: String,
    slug: String,
) -> Result<Value, FetchError> {
    let query = query(username, slug).await;
    super::use_query::fetch_gql_data(query).await
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
    super::use_query::fetch_gql_data(query).await
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/article.graphql",
    response_derives = "Debug, Clone"
)]
struct ArticleDataById;

pub async fn fetch_article_data_by_id(id: String) -> Result<Value, FetchError> {
    let veriables = article_data_by_id::Variables { id };
    let query_body = ArticleDataById::build_query(veriables);
    let query = json!(query_body);
    super::use_query::fetch_gql_data(query).await
}

type ObjectId = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/article.graphql",
    response_derives = "Debug, Clone"
)]
struct ArticleUpdate;

pub async fn update_article_data(
    article_id: ObjectId,
    user_id: ObjectId,
    subject: String,
    category_id: ObjectId,
    summary: String,
    content: String,
    published: bool,
    top: bool,
    recommended: bool,
    token: String,
) -> Result<Value, FetchError> {
    let veriables = article_update::Variables {
        article_id,
        user_id,
        subject,
        category_id,
        summary,
        content,
        published,
        top,
        recommended,
        token,
    };
    let query_body = ArticleUpdate::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;

    Ok(data)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/article.graphql",
    response_derives = "Debug, Clone"
)]
struct ArticleNew;

pub async fn new_article_data(
    user_id: ObjectId,
    subject: String,
    category_id: ObjectId,
    summary: String,
    content: String,
    published: bool,
    top: bool,
    recommended: bool,
) -> Result<Value, FetchError> {
    let veriables = article_new::Variables {
        user_id,
        subject,
        category_id,
        summary,
        content,
        published,
        top,
        recommended,
    };
    let query_body = ArticleNew::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;

    Ok(data)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/article.graphql",
    response_derives = "Debug, Clone"
)]
struct ArticleDelete;

pub async fn delete_article_data(article_id: ObjectId, token: String) -> Result<Value, FetchError> {
    let veriables = article_delete::Variables { article_id, token };
    let query_body = ArticleDelete::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;

    Ok(data)
}

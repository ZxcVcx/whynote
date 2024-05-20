use graphql_client::GraphQLQuery;
use serde_json::{json, Value};

use crate::utils;

use super::use_query::FetchError;
type DateTime = String;
type ObjectId = String;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/topicslist.graphql",
    response_derives = "Debug, Clone"
)]
pub struct TopicsList;

pub async fn fetch_topics_list() -> Result<Value, FetchError> {
    let veriables = topics_list::Variables {};
    let query_body = TopicsList::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;

    Ok(data)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/topic.graphql",
    response_derives = "Debug, Clone"
)]
pub struct TopicUpdate;

pub async fn update_topic_data(
    topic_id: ObjectId,
    name: ObjectId,
) -> Result<Value, FetchError> {
    let veriables = topic_update::Variables {
        topic_id,
        name,
        token: utils::storage::get_pair_value("jwt").unwrap(),
    };
    let query_body = TopicUpdate::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;

    Ok(data)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/topic.graphql",
    response_derives = "Debug, Clone"
)]
pub struct TopicDelete;

pub async fn delete_topic_data(topic_id: ObjectId) -> Result<Value, FetchError> {
    let veriables = topic_delete::Variables {
        topic_id,
        token: utils::storage::get_pair_value("jwt").unwrap(),
    };
    let query_body = TopicDelete::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;
    Ok(data)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/topic.graphql",
    response_derives = "Debug, Clone"
)]
pub struct TopicNewByToken;
pub async fn create_topic_by_token(
    name: String,
) -> Result<Value, FetchError> {
    let veriables = topic_new_by_token::Variables {
        name,
        token: utils::storage::get_pair_value("jwt").unwrap(),
    };
    let query_body = TopicNewByToken::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;
    Ok(data)
}


use graphql_client::GraphQLQuery;
use serde_json::{json, Value};

use super::use_query::FetchError;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug, Clone"
)]
pub struct UserDataByUsername;

pub async fn fetch_user_data_by_username(username: String) -> Result<Value, FetchError> {
    let veriables = user_data_by_username::Variables { username };
    let query_body = UserDataByUsername::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;
    Ok(data)
}

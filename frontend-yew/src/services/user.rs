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

type ObjectId = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug, Clone"
)]
pub struct UserDataByEmail;

pub async fn fetch_user_data_by_email(email: String) -> Result<Value, FetchError> {
    let veriables = user_data_by_email::Variables { email };
    let query_body = UserDataByEmail::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;
    Ok(data)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/user.graphql",
    response_derives = "Debug, Clone"
)]
struct UserUpdateProfile;

pub async fn user_update_profile_data(
    email: String,
    username: String,
    nickname: String,
    blog_name: String,
    website: String,
    bio: String,
    token: String,
) -> Result<Value, FetchError> {
    let veriables = user_update_profile::Variables {
        email,
        username,
        nickname,
        cred: "anyway".to_string(),
        blog_name,
        website,
        bio,
        token,
    };
    let query_body = UserUpdateProfile::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;
    Ok(data)
}

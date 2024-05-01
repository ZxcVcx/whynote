use graphql_client::GraphQLQuery;
use serde_json::{json, Value};

use super::use_query::FetchError;

// type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/signin.graphql"
)]
struct SignInData;

async fn sign_in_query(signature: String, password: String) -> Value {
    let veriables = sign_in_data::Variables {
        signature,
        password,
    };
    let query_body = SignInData::build_query(veriables);
    let query = json!(query_body);

    query
}

pub async fn fetch_sign_in_data(signature: String, password: String) -> Result<Value, FetchError> {
    let query = sign_in_query(signature, password).await;
    let data = super::use_query::fetch_gql_data(query).await;
    data
}
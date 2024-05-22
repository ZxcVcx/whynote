use graphql_client::GraphQLQuery;
use serde_json::{json, Value};

use crate::utils;

use super::use_query::FetchError;
type DateTime = String;
type ObjectId = String;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/categorieslist.graphql",
    response_derives = "Debug, Clone"
)]
pub struct CategoriesList;

pub async fn fetch_categories_list() -> Result<Value, FetchError> {
    let veriables = categories_list::Variables {};
    let query_body = CategoriesList::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;

    Ok(data)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/category.graphql",
    response_derives = "Debug, Clone"
)]
pub struct CategoryData;

pub async fn fetch_category_data_by_slug(slug: String) -> Result<Value, FetchError> {
    let veriables = category_data::Variables { slug };
    let query_body = CategoryData::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;

    Ok(data)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/category.graphql",
    response_derives = "Debug, Clone"
)]
pub struct CategoryUpdate;

pub async fn update_category_data(
    category_id: ObjectId,
    name: ObjectId,
    description: String,
) -> Result<Value, FetchError> {
    let veriables = category_update::Variables {
        category_id,
        name,
        description,
        token: utils::storage::get_pair_value("jwt").unwrap(),
    };
    let query_body = CategoryUpdate::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;

    Ok(data)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/category.graphql",
    response_derives = "Debug, Clone"
)]
pub struct CategoryDelete;

pub async fn delete_category_data(category_id: ObjectId) -> Result<Value, FetchError> {
    let veriables = category_delete::Variables {
        category_id,
        token: utils::storage::get_pair_value("jwt").unwrap(),
    };
    let query_body = CategoryDelete::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;
    Ok(data)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/category.graphql",
    response_derives = "Debug, Clone"
)]
pub struct CategoryNewByToken;

pub async fn create_category_by_token(
    name: String,
    description: String,
) -> Result<Value, FetchError> {
    let veriables = category_new_by_token::Variables {
        name,
        description,
        token: utils::storage::get_pair_value("jwt").unwrap(),
    };
    let query_body = CategoryNewByToken::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;
    Ok(data)
}

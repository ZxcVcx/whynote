use graphql_client::GraphQLQuery;
use serde_json::{json, Value};

use super::use_query::FetchError;
type DateTime = String;

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

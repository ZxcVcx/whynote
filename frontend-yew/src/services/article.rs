use graphql_client::GraphQLQuery;
use serde_json::{json, Value};

type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/article.graphql"
)]
struct ArticleData;

// https://blog.lanesawyer.dev/32266/yew-hooks-with-graphql

async fn query(username: String, slug: String) -> Value {
    let build_query = ArticleData::build_query(article_data::Variables { username, slug });
    let query = json!(build_query);
    query
}

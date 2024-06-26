pub mod mutations;
pub mod queries;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};

use crate::gql::mutations::MutationRoot;
use crate::gql::queries::QueryRoot;
use crate::{dbs::mongo, AppState};
use axum::{
    extract::{Json as AxumJson, State},
    response::Html,
    Json,
};

use crate::utils::constants::CFG;
use std::sync::Arc;

pub async fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    let mongo_ds = mongo::DataSource::init().await;
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(mongo_ds)
    .finish()
}

/// GraphQL endpoint
/// API与Endpoint的区别，你知道多少？
/// HTTP is commonly associated with REST,
/// which uses “resources” as its core concept.
/// In contrast, GraphQL’s conceptual model is an entity graph.
/// As a result, entities in GraphQL are not identified by URLs.
/// Instead, a GraphQL server operates on a single URL/endpoint,
/// usually /graphql, and all GraphQL requests for a
/// given service should be directed at this endpoint.
pub async fn graphql(
    State(state): State<Arc<AppState>>,
    req: AxumJson<async_graphql::Request>,
) -> Json<async_graphql::Response> {
    let schema = state.schema.clone();
    let gql_resp = schema.execute(req.0).await;
    Json(gql_resp)
}

/// GraphQL Playground
/// https://github.com/graphql/graphiql
/// GraphQL IDE Monorepo for all platforms.
pub async fn graphiql() -> Html<String> {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        CFG.get("GQL_PATH").unwrap(),
    )))
}

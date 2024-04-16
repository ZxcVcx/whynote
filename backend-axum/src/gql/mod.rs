pub mod queries;
pub mod mutations;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};

use axum::{extract::{Json as AxumJson, State}, response::Html, Json};
use crate::{dbs::mongo, AppState};
use crate::gql::queries::QueryRoot;
use crate::utils::constants::CFG;
use std::sync::Arc;

use self::mutations::MutationRoot;

pub async fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    let mongo_ds = mongo::DataSource::init().await;
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(mongo_ds)
        .finish()
}


pub async fn graphql(State(state): State<Arc<AppState>> , req: AxumJson<async_graphql::Request>) -> Json<async_graphql::Response> {
    let schema = state.schema.clone();
    let gql_resp = schema.execute(req.0).await;
    Json(gql_resp)
}

// pub async fn graphql(req: Request<State>) -> tide::Result {
//     let schema = req.state().schema.clone();
//     let gql_resp = schema.execute(receive_json(req).await?).await;

//     let mut resp = Response::new(StatusCode::Ok);
//     resp.set_body(Body::from_json(&gql_resp)?);

//     Ok(resp.into())
// }

pub async fn graphiql() -> Html<String> {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        CFG.get("GQL_PATH").unwrap(),
    )))
}


// pub async fn graphiql(_: Request<State>) -> tide::Result {
//     let mut resp = Response::new(StatusCode::Ok);
//     resp.set_body(playground_source(GraphQLPlaygroundConfig::new(
//         CFG.get("GQL_PATH").unwrap(),
//     )));
//     resp.set_content_type(mime::HTML);

//     Ok(resp.into())
// }





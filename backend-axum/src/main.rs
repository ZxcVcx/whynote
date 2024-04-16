use axum::{ body::Body, http::{Response, StatusCode}, middleware::{self, Next}, routing::{get, post}, Router};
// use tower_http::trace::TraceLayer;
use std::sync::Arc;
mod gql;
mod utils;
mod users;
mod dbs;

#[derive(Clone)]
pub struct AppState {
    pub schema: async_graphql::Schema<
        gql::queries::QueryRoot,
        gql::mutations::MutationRoot,
        async_graphql::EmptySubscription,
    >,
}

use crate::gql::{build_schema, graphiql, graphql};
use crate::utils::constants::CFG;
use axum::extract::Request;


async fn log_requests(req: Request, next: Next) -> Result<Response<Body>, StatusCode> {
    let path = req.uri().path().to_owned();
    let method = req.method().to_string();
    tracing::info!("Incoming request: {} {}", method, path);

    let start = std::time::Instant::now();
    let res = next.run(req).await;
    let status = res.status();
    if status.is_client_error() {
        tracing::warn!("{} {} failed with status: {} {:?}", method, path, status, start.elapsed());
    } else {
        tracing::info!("{} {} succeeded with status: {} {:?}", method, path, status, start.elapsed());
    }
    // if status.is_server_error() {
    //     if let Some(error) = res.
    // }
    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    // Set up the global logger
    tracing_subscriber::fmt::init();


    // let schema = Arc::new(build_schema().await);
    let schema = build_schema().await;
    let shared_state = Arc::new(AppState { schema });
    // let app_state = AppState {schema : schema};
    let app = Router::new()
        .route("/graphql", post(graphql))
        .route("/graphiql", get(graphiql))
        // .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(log_requests))
        .with_state(shared_state);
        // .layer(TraceLayer::new_for_http());

    let addr = CFG.get("ADDR").unwrap();
    let port = CFG.get("PORT").unwrap();
    let addr = format!("{}:{}", addr, port);

    let listener = tokio::net::TcpListener::bind(&addr).await.expect("Linstener failed to bind");
    tracing::info!("Listening on: {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
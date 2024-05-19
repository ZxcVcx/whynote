use axum::{
    http::{HeaderValue, Method},
    middleware::{self},
    routing::{get, post},
    Router,
};
use backend_axum::{gql, utils, AppState};
// use tower_http::trace::TraceLayer;
use std::sync::Arc;

use gql::{build_schema, graphiql, graphql};
use tower_http::cors::CorsLayer;
use utils::constants::CFG;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Set up the global logger
    tracing_subscriber::fmt::init();

    // let schema = Arc::new(build_schema().await);
    let schema = build_schema().await;
    let shared_state = Arc::new(AppState { schema });
    // let app_state = AppState {schema : schema};
    let app = Router::new()
        // .route("/graphql", post(graphql))
        .route(CFG["GQL_PATH"].as_str(), post(graphql))
        .route(CFG["GIQL_PATH"].as_str(), get(graphiql))
        // .route("/graphiql", get(graphiql))
        .layer(middleware::from_fn(backend_axum::log_requests))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers(tower_http::cors::Any)
                .allow_credentials(false),
        )
        .with_state(shared_state);
    // .layer(TraceLayer::new_for_http());

    let addr = CFG.get("ADDR").unwrap();
    let port = CFG.get("PORT").unwrap();
    let addr = format!("{}:{}", addr, port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Linstener failed to bind");
    tracing::info!("Listening on: {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

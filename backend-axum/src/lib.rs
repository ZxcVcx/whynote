pub mod articles;
pub mod categories;
pub mod dbs;
pub mod gql;
pub mod topics;
pub mod users;

pub mod utils;

// custom mods
pub mod wishes;

use axum::extract::Request;
use axum::{
    body::Body,
    http::{Response, StatusCode},
    middleware::Next,
};

#[derive(Clone)]
pub struct AppState {
    pub schema: async_graphql::Schema<
        gql::queries::QueryRoot,
        gql::mutations::MutationRoot,
        async_graphql::EmptySubscription,
    >,
}

// https://github.com/tokio-rs/axum/blob/main/examples/consume-body-in-extractor-or-middleware/src/main.rs
pub async fn log_requests(req: Request, next: Next) -> Result<Response<Body>, StatusCode> {
    let path = req.uri().path().to_owned();
    let method = req.method().to_string();
    tracing::info!("Incoming request: {} {}", method, path);

    let start = std::time::Instant::now();
    let res = next.run(req).await;
    let status = res.status();
    if status.is_client_error() {
        tracing::warn!(
            "{} {} failed with status: {} {:?}",
            method,
            path,
            status,
            start.elapsed()
        );
    } else {
        tracing::info!(
            "{} {} succeeded with status: {} {:?}",
            method,
            path,
            status,
            start.elapsed()
        );
    }
    Ok(res)
}

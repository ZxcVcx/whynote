mod dbs;
mod gql;
mod users;
mod utils;

use crate::gql::{build_schema, graphiql, graphql};
use crate::utils::constants::CFG;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // 初始 Tide 应用程序状态
    let schema = build_schema().await;
    let app_state = State { schema: schema };
    let mut app = tide::with_state(app_state);

    // 路由配置
    app.at(CFG.get("GQL_PATH").unwrap()).post(graphql);
    app.at(CFG.get("GIQL_PATH").unwrap()).get(graphiql);

    app.listen(format!(
        "{}:{}",
        CFG.get("ADDR").unwrap(),
        CFG.get("PORT").unwrap()
    ))
    .await?;

    // app.listen(format!("{}:{}", "127.0.0.1", "8080")).await?;
    Ok(())
}

//  Tide 应用程序作用域状态 state.
#[derive(Clone)]
pub struct State {
    pub schema: async_graphql::Schema<
        gql::queries::QueryRoot,
        gql::mutations::MutationRoot,
        async_graphql::EmptySubscription,
    >,
}

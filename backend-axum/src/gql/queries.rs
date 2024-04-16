use async_graphql::Context;

use crate::{
    dbs::mongo::DataSource,
    users::{self, models::User},
    utils::constants::GqlResult,
};
pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn all_users(&self, ctx: &Context<'_>) -> GqlResult<Vec<User>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::all_users(db).await
    }

    async fn get_user_by_email(&self, ctx: &Context<'_>, email: String) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::get_user_by_email(db, &email).await
    }
}

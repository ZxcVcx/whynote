use async_graphql::Context;

use crate::{
    dbs::mongo::DataSource,
    utils::constants::GqlResult,
    wishes::{self, models::Wish},
};

#[derive(Default)]
pub struct WishQuery;

#[async_graphql::Object]
impl WishQuery {
    // get all wishes
    async fn wishes(&self, ctx: &Context<'_>, published: i32) -> GqlResult<Vec<Wish>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        wishes::services::wishes(db, published).await
    }

    // get random wish
    async fn random_wish(&self, ctx: &Context<'_>, username: String) -> GqlResult<Wish> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        wishes::services::random_wish(db, &username).await
    }
}

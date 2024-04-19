use async_graphql::Context;

use super::services;
use crate::{
    dbs::mongo::DataSource,
    utils::constants::GqlResult,
    wishes::models::{Wish, WishNew},
};

#[derive(Default)]
pub struct WishMutation;

#[async_graphql::Object]
impl WishMutation {
    async fn wish_new(&self, ctx: &Context<'_>, wish_new: WishNew) -> GqlResult<Wish> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        services::wish_new(db, wish_new).await
    }
}

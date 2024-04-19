use async_graphql::Context;

use super::models::*;
use crate::{dbs::mongo::DataSource, utils::constants::GqlResult};

#[derive(Default)]
pub struct CategoryMutation;

#[async_graphql::Object]
impl CategoryMutation {
    // Add new category
    async fn category_new(
        &self,
        ctx: &Context<'_>,
        category_new: CategoryNew,
    ) -> GqlResult<Category> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::category_new(db, category_new).await
    }

    // Add new category
    async fn category_user_new(
        &self,
        ctx: &Context<'_>,
        category_user_new: CategoryUserNew,
    ) -> GqlResult<CategoryUser> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::category_user_new(db, category_user_new).await
    }
}

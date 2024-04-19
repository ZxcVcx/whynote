use async_graphql::Context;
use bson::oid::ObjectId;

use crate::{categories::models::Category, dbs::mongo::DataSource, utils::constants::GqlResult};

#[derive(Default)]
pub struct CategoryQuery;

#[async_graphql::Object]
impl CategoryQuery {
    // Get all categories
    async fn categories(&self, ctx: &Context<'_>) -> GqlResult<Vec<Category>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::categories(db).await
    }

    // Get all categories by user_id
    async fn categories_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: ObjectId,
    ) -> GqlResult<Vec<Category>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::categories_by_user_id(db, user_id).await
    }

    // Get all categories by username
    async fn categories_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<Vec<Category>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::categories_by_username(db, &username).await
    }

    // Get category by its id
    async fn category_by_id(&self, ctx: &Context<'_>, id: ObjectId) -> GqlResult<Category> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::category_by_id(db, id).await
    }

    // Get category by its slug
    async fn category_by_slug(&self, ctx: &Context<'_>, slug: String) -> GqlResult<Category> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::category_by_slug(db, &slug).await
    }
}

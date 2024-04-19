use async_graphql::Context;
use bson::oid::ObjectId;

use crate::{categories::{self, models::Category}, dbs::mongo::DataSource, topics::models::Topic, utils::constants::GqlResult};

#[derive(Default)]
pub struct TopicMutation;

#[async_graphql::Object]
impl TopicMutation {
    // Get category by its id
    async fn category_by_id(&self, ctx: &Context<'_>, id: ObjectId) -> GqlResult<Category> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        categories::services::category_by_id(db, id).await
    }

    // Get category by its slug
    async fn category_by_slug(&self, ctx: &Context<'_>, slug: String) -> GqlResult<Category> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        categories::services::category_by_slug(db, &slug).await
    }

    // get all topics
    async fn topics(&self, ctx: &Context<'_>) -> GqlResult<Vec<Topic>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::topics(db).await
    }

    // get topic info by id
    async fn topic_by_id(&self, ctx: &Context<'_>, id: ObjectId) -> GqlResult<Topic> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::topic_by_id(db, id).await
    }

    // get topic info by slug
    async fn topic_by_slug(&self, ctx: &Context<'_>, slug: String) -> GqlResult<Topic> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::topic_by_slug(db, &slug).await
    }

    // get topics by article_id
    async fn topics_by_article_id(
        &self,
        ctx: &Context<'_>,
        article_id: ObjectId,
    ) -> GqlResult<Vec<Topic>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::topics_by_article_id(db, article_id).await
    }

    // get topics by user_id
    async fn topics_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: ObjectId,
    ) -> GqlResult<Vec<Topic>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::topics_by_user_id(db, user_id).await
    }

    // get topics by username
    async fn topics_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<Vec<Topic>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::topics_by_username(db, &username).await
    }

    // get topics by category_id
    async fn topics_by_category_id(
        &self,
        ctx: &Context<'_>,
        category_id: ObjectId,
        published: i32,
    ) -> GqlResult<Vec<Topic>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::topics_by_category_id(db, category_id, published).await
    }
}

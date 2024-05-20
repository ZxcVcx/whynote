use async_graphql::Context;
use bson::oid::ObjectId;
// use bson::oid::ObjectId;

use crate::{
    dbs::mongo::DataSource,
    topics::{
        self,
        models::{Topic, TopicArticle, TopicArticleNew, TopicNew},
    },
    utils::constants::GqlResult,
};

#[derive(Default)]
pub struct TopicMutation;

#[async_graphql::Object]
impl TopicMutation {
    // Add new topics
    async fn topics_new(&self, ctx: &Context<'_>, topic_names: String) -> GqlResult<Vec<Topic>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        topics::services::topics_new(db, &topic_names).await
    }

    // Add new topic
    async fn topic_new(&self, ctx: &Context<'_>, topic_new: TopicNew) -> GqlResult<Topic> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        topics::services::topic_new(db, topic_new).await
    }

    // Add new topic_article
    async fn topic_article_new(
        &self,
        ctx: &Context<'_>,
        topic_article_new: TopicArticleNew,
    ) -> GqlResult<TopicArticle> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        topics::services::topic_article_new(db, topic_article_new).await
    }

    async fn topic_new_by_token(
        &self,
        ctx: &Context<'_>,
        topic_new: TopicNew,
        token: String,
    ) -> GqlResult<Topic> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        topics::services::topic_new_by_token(db, topic_new, &token).await
    }

    async fn topic_delete(
        &self,
        ctx: &Context<'_>,
        topic_id: ObjectId,
        token: String,
    ) -> GqlResult<Topic> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        topics::services::topic_delete(db, topic_id, &token).await
    }

    async fn topic_update(
        &self,
        ctx: &Context<'_>,
        topic_id: ObjectId,
        topic_new: TopicNew,
        token: String,
    ) -> GqlResult<Topic> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        topics::services::topic_update(db, topic_id, topic_new, &token).await
    }
}

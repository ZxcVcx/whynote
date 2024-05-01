use async_graphql::Context;
use bson::oid::ObjectId;

use crate::articles::models::Article;
use crate::{dbs::mongo::DataSource, utils::constants::GqlResult};

use crate::articles;

#[derive(Default)]
pub struct ArticleQuery;

#[async_graphql::Object]
impl ArticleQuery {
    // Get article by its slug
    async fn article_by_username_and_slug(
        &self,
        ctx: &Context<'_>,
        username: String,
        slug: String,
    ) -> GqlResult<Article> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        articles::services::article_by_username_and_slug(db, &username, &slug).await
    }

    async fn article_by_slug(
        &self,
        ctx: &Context<'_>,
        slug: String,
    ) -> GqlResult<Article> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        articles::services::article_by_slug(db, &slug).await
    }


    // Get all articles
    async fn articles(&self, ctx: &Context<'_>, published: bool) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        articles::services::articles(db, published).await
    }

    async fn articles_in_position(
        &self,
        ctx: &Context<'_>,
        username: String,
        position: String,
        limit: i64,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        articles::services::articles_in_position(db, &username, &position, limit).await
    }

    // Get all articles of one user by user_id
    async fn articles_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: ObjectId,
        published: bool,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        articles::services::articles_by_user_id(db, user_id, published).await
    }

    // Get all articles of one user by username
    async fn articles_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
        published: bool,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        articles::services::articles_by_username(db, &username, published).await
    }

    // Get all articles by category_id
    async fn articles_by_category_id(
        &self,
        ctx: &Context<'_>,
        category_id: ObjectId,
        published: bool,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        articles::services::articles_by_category_id(db, category_id, published).await
    }

    // Get all articles by topic_id
    async fn articles_by_topic_id(
        &self,
        ctx: &Context<'_>,
        topic_id: ObjectId,
        published: bool,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        articles::services::articles_by_topic_id(db, topic_id, published).await
    }

    // async fn articles_in_position_paged(
    //     &self,
    //     ctx: &Context<'_>,
    //     username: String,
    //     position: String,
    //     limit: i64,
    //     skip: i64,
    // ) -> GqlResult<Vec<Article>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     articles::services::articles_in_position_paged(db, &username, &position, limit, skip).await
    // }

    // async fn articles_by_username_paged(
    //     &self,
    //     ctx: &Context<'_>,
    //     username: String,
    //     published: bool,
    //     limit: i64,
    //     skip: i64,
    // ) -> GqlResult<Vec<Article>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     articles::services::articles_by_username_paged(db, &username, published, limit, skip).await
    // }

    // async fn articles_by_category_id_paged(
    //     &self,
    //     ctx: &Context<'_>,
    //     category_id: ObjectId,
    //     published: bool,
    //     limit: i64,
    //     skip: i64,
    // ) -> GqlResult<Vec<Article>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     articles::services::articles_by_category_id_paged(db, category_id, published, limit, skip).await
    // }

    // async fn articles_by_topic_id_paged(
    //     &self,
    //     ctx: &Context<'_>,
    //     topic_id: ObjectId,
    //     published: bool,
    //     limit: i64,
    //     skip: i64,
    // ) -> GqlResult<Vec<Article>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     articles::services::articles_by_topic_id_paged(db, topic_id, published, limit, skip).await
    // }
}

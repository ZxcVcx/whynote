use async_graphql::Context;
use bson::oid::ObjectId;

use crate::{
    dbs::mongo::DataSource,
    users::{self, models::{SignInfo, User}},
    utils::constants::GqlResult, wishes::{self, models::Wish},
};
pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn users(&self, ctx: &Context<'_>, token: String) -> GqlResult<Vec<User>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::users(db, &token).await
    }

    async fn user_by_email(&self, ctx: &Context<'_>, email: String) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::user_by_email(db, &email).await
    }

    async fn user_by_id(&self, ctx: &Context<'_>, id: ObjectId) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::user_by_id(db, id).await
    }

    async fn user_by_username(&self, ctx: &Context<'_>, username: String) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::user_by_username(db, &username).await
    }

    async fn user_sign_in(
        &self,
        ctx: &Context<'_>,
        signature: String,
        cred: String,
    ) -> GqlResult<SignInfo> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::user_sign_in(db, &signature, &cred).await
    }

    // async fn articles(&self, ctx: &Context<'_>, published: i32) -> GqlResult<Vec<Article>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     articles::services::articles(db, published).await
    // }

    // async fn article_by_slug(
    //     &self,
    //     ctx: &Context<'_>,
    //     username: String,
    //     slug: String,
    // ) -> GqlResult<Article> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     articles::services::article_by_slug(db, &username, &slug).await
    // }

    // async fn article_in_position(
    //     &self,
    //     ctx: &Context<'_>,
    //     username: String,
    //     position: String,
    //     limit: i64,
    // ) -> GqlResult<Article> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     articles::services::article_in_position(db, &username, &position, limit).await
    // }

    // async fn articles_by_user_id(
    //     &self,
    //     ctx: &Context<'_>,
    //     user_id: ObjectId,
    //     published: i32,
    // ) -> GqlResult<Vec<Article>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     articles::services::articles_by_user_id(db, &user_id).await
    // }

    // async fn articles_by_username(
    //     &self,
    //     ctx: &Context<'_>,
    //     username: String,
    //     published: i32,
    // ) -> GqlResult<Vec<Article>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     articles::services::articles_by_username(db, &username, published).await
    // }

    // async fn articles_by_category_id(
    //     &self,
    //     ctx: &Context<'_>,
    //     category_id: ObjectId,
    //     published: i32,
    // ) -> GqlResult<Vec<Article>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     articles::services::articles_by_category_id(db, &category_id, published).await
    // }

    // async fn articles_by_category_name(
    //     &self,
    //     ctx: &Context<'_>,
    //     category_name: String,
    //     published: i32,
    // ) -> GqlResult<Vec<Article>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     articles::services::articles_by_category_name(db, &category_name, published).await
    // }

    // async fn articles_by_topic_id(
    //     &self,
    //     ctx: &Context<'_>,
    //     topic_id: ObjectId,
    //     published: i32,
    // ) -> GqlResult<Vec<Article>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     articles::services::articles_by_topic_id(db, &topic_id, published).await
    // }

    // async fn categories(&self, ctx: &Context<'_>) -> GqlResult<Vec<Category>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     categories::services::categories(db).await
    // }

    // async fn category_by_user_id(
    //     &self,
    //     ctx: &Context<'_>,
    //     user_id: ObjectId,
    // ) -> GqlResult<Category> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     categories::services::category_by_user_id(db, &user_id).await
    // }

    // async fn category_by_username(
    //     &self,
    //     ctx: &Context<'_>,
    //     username: String,
    // ) -> GqlResult<Category> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     categories::services::category_by_username(db, &username).await
    // }

    // async fn category_by_id(&self, ctx: &Context<'_>, id: ObjectId) -> GqlResult<Category> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     categories::services::category_by_id(db, id).await
    // }

    // // Get category by its slug
    // async fn category_by_slug(&self, ctx: &Context<'_>, slug: String) -> GqlResult<Category> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     categories::services::category_by_slug(db, &slug).await
    // }

    // // get all topics
    // async fn topics(&self, ctx: &Context<'_>) -> GqlResult<Vec<Topic>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     topics::services::topics(db).await
    // }

    // // get topic info by id
    // async fn topic_by_id(&self, ctx: &Context<'_>, id: ObjectId) -> GqlResult<Topic> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     topics::services::topic_by_id(db, id).await
    // }

    // // get topic info by slug
    // async fn topic_by_slug(&self, ctx: &Context<'_>, slug: String) -> GqlResult<Topic> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     topics::services::topic_by_slug(db, &slug).await
    // }

    // // get topics by article_id
    // async fn topics_by_article_id(
    //     &self,
    //     ctx: &Context<'_>,
    //     article_id: ObjectId,
    // ) -> GqlResult<Vec<Topic>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     topics::services::topics_by_article_id(db, article_id).await
    // }

    // // get topics by user_id
    // async fn topics_by_user_id(
    //     &self,
    //     ctx: &Context<'_>,
    //     user_id: ObjectId,
    // ) -> GqlResult<Vec<Topic>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     topics::services::topics_by_user_id(db, user_id).await
    // }

    // // get topics by username
    // async fn topics_by_username(
    //     &self,
    //     ctx: &Context<'_>,
    //     username: String,
    // ) -> GqlResult<Vec<Topic>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     topics::services::topics_by_username(db, &username).await
    // }

    // // get topics by category_id
    // async fn topics_by_category_id(
    //     &self,
    //     ctx: &Context<'_>,
    //     category_id: ObjectId,
    //     published: i32,
    // ) -> GqlResult<Vec<Topic>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     topics::services::topics_by_category_id(db, category_id, published).await
    // }

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

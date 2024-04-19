use async_graphql::Context;

use crate::{
    articles::models::{Article, ArticleNew},
    dbs::mongo::DataSource,
    utils::constants::GqlResult,
};

#[derive(Default)]
pub struct ArticleMutation;

#[async_graphql::Object]
impl ArticleMutation {
    // Add new article
    async fn article_new(&self, ctx: &Context<'_>, article_new: ArticleNew) -> GqlResult<Article> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::article_new(db, article_new).await
    }

    // async fn article_delete(
    //     &self,
    //     ctx: &Context<'_>,
    //     article_id: ObjectId,
    //     token: String,
    // ) -> GqlResult<Article> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     services::article_delete(db, article_id, &token).await
    // }

    // async fn article_update(
    //     &self,
    //     ctx: &Context<'_>,
    //     article_id: ObjectId,
    //     article_new: ArticleNew,
    //     token: String,
    // ) -> GqlResult<Article> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     services::article_update(db, article_id, article_new, &token).await
    // }
}

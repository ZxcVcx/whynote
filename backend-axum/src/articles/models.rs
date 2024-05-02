use async_graphql::SimpleObject;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::categories::models::Category;
use crate::dbs::mongo::DataSource;
use crate::topics::models::Topic;
use crate::utils::constants::GqlResult;
use crate::{categories, topics};
// use crate::categories::{self, models::Category};
// use crate::topics::{self, models::Topic};
use crate::users::{self, models::User};

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
#[graphql(complex)]
pub struct Article {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub subject: String,
    pub category_id: ObjectId,
    pub summary: String,
    pub slug: String,
    pub uri: String,
    pub content: String,
    pub published: bool,
    pub top: bool,
    pub recommended: bool,
    // #[graphql()]
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[async_graphql::ComplexObject]
impl Article {
    pub async fn content_html(&self) -> String {
        use pulldown_cmark::{html, Options, Parser};

        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);

        let parser = Parser::new_ext(&self.content, options);

        let mut content_html = String::new();
        html::push_html(&mut content_html, parser);

        content_html
    }

    pub async fn user(&self, ctx: &async_graphql::Context<'_>) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::user_by_id(db, self.user_id).await
    }

    pub async fn category(&self, ctx: &async_graphql::Context<'_>) -> GqlResult<Category> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        categories::services::category_by_id(db, self.category_id).await
    }

    pub async fn topics(&self, ctx: &async_graphql::Context<'_>) -> GqlResult<Vec<Topic>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        topics::services::topics_by_article_id(db, self.id).await
    }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct ArticleNew {
    pub user_id: ObjectId,
    pub subject: String,
    pub category_id: ObjectId,
    pub summary: String,
    #[graphql(skip)]
    pub slug: String,
    #[graphql(skip)]
    pub uri: String,
    pub content: String,
    #[graphql(skip)]
    pub published: bool,
    #[graphql(skip)]
    pub top: bool,
    #[graphql(skip)]
    pub recommended: bool,
}

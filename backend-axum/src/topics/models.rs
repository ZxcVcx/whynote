use async_graphql::SimpleObject;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::articles::{models::Article, services::articles_by_topic_id};
use crate::dbs::mongo::DataSource;
use crate::utils::constants::GqlResult;

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
#[graphql(complex)]
pub struct Topic {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub quotes: i64,
    pub slug: String,
    pub uri: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[async_graphql::ComplexObject]
impl Topic {
    pub async fn articles(&self, ctx: &async_graphql::Context<'_>) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        articles_by_topic_id(db, self.id, 1).await
    }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct TopicNew {
    pub name: String,
    #[graphql(skip)]
    pub quotes: i64,
    #[graphql(skip)]
    pub slug: String,
    #[graphql(skip)]
    pub uri: String,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct TopicArticle {
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub article_id: ObjectId,
    pub topic_id: ObjectId,
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct TopicArticleNew {
    pub user_id: ObjectId,
    pub article_id: ObjectId,
    pub topic_id: ObjectId,
}

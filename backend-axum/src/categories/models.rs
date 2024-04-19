use crate::dbs::mongo::DataSource;
use crate::utils::constants::GqlResult;
use crate::{
    articles::{models::Article, services::articles_by_category_id},
    topics::{models::Topic, services::topics_by_category_id},
};
use async_graphql::SimpleObject;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
#[graphql(complex)]
pub struct Category {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub quotes: i64,
    pub slug: String,
    pub uri: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[async_graphql::ComplexObject]
impl Category {
    pub async fn articles(&self, ctx: &async_graphql::Context<'_>) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        articles_by_category_id(db, self.id, 1).await
    }

    pub async fn topics(&self, ctx: &async_graphql::Context<'_>) -> GqlResult<Vec<Topic>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        topics_by_category_id(db, self.id, 1).await
    }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct CategoryNew {
    pub name: String,
    pub description: String,
    #[graphql(skip)]
    pub quotes: i64,
    #[graphql(skip)]
    pub slug: String,
    #[graphql(skip)]
    pub uri: String,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct CategoryUser {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub category_id: ObjectId,
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct CategoryUserNew {
    pub user_id: ObjectId,
    pub category_id: ObjectId,
}

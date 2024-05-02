use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::{
    articles::{models::Article, services::articles_by_user_id},
    dbs::mongo::DataSource,
    utils::constants::GqlResult,
};
// /// Simple Object
// #[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
// pub struct User {
//     #[serde(rename = "_id")]
//     pub id: ObjectId,
//     pub email: String,
//     pub username: String,
//     pub cred: String,
// }

/// Complex Object
/// https://async-graphql.github.io/async-graphql/en/define_simple_object.html#user-defined-resolvers
#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
// https://docs.rs/async-graphql/latest/async_graphql/derive.SimpleObject.html
#[graphql(complex)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub email: String,
    pub username: String,
    pub nickname: String,
    pub picture: String,
    pub cred: String,
    pub blog_name: String,
    pub website: String,
    pub bio: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub banned: bool,
}

#[async_graphql::ComplexObject]
impl User {
    pub async fn articles(
        &self,
        ctx: &async_graphql::Context<'_>,
        published: bool,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        articles_by_user_id(db, self.id, published).await
    }

    // pub async fn elsewhere(
    //     &self,
    //     ctx: &async_graphql::Context<'_>,
    // ) -> GqlResult<Vec<Link>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     links_by_user_id(db, self.id).await
    //     // format!("https://www.elsewhere.com/{}", self.username)
    // }
}

/// New User
#[derive(async_graphql::InputObject, Serialize, Deserialize, Clone)]
pub struct UserNew {
    pub email: String,
    pub username: String,
    pub nickname: String,
    pub cred: String,
    pub blog_name: String,
    pub website: String,
    pub bio: String,
    #[graphql(skip)]
    pub banned: bool,
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone)]
pub struct SignInfo {
    pub email: String,
    pub username: String,
    pub token: String,
}

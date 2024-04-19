use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
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
    // pub async fn articles(
    //     &self,
    //     ctx: &async_graphql::Context<'_>,
    //     published: i32,
    // ) -> GqlResult<Vec<Article>> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     articles_by_user_id(db, self._id, published).await
    // }
}

//// Normal Object
// #[derive(Serialize, Deserialize, Clone)]
// pub struct User {
//     #[serde(rename = "_id")]
//     pub id: ObjectId,
//     pub email: String,
//     pub username: String,

//     pub cred: String,
// }

// #[async_graphql::Object]
// impl User {
//     pub async fn id(&self) -> ObjectId {
//         self.id.clone()
//     }
//     pub async fn email(&self) -> &str {
//         self.email.as_str()
//     }
//     pub async fn username(&self) -> &str {
//         self.username.as_str()
//     }
//     pub async fn cred(&self) -> &str {
//         self.cred.as_str()
//     }
// }

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


use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use crate::{dbs::mongo::DataSource, users, utils::constants::GqlResult};


#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone)]
#[graphql(complex)]
pub struct Wish {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub aphorism: String,
    pub author: String,
    pub published: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[async_graphql::ComplexObject]
impl Wish {
    pub async fn user(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<users::models::User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::user_by_id(db, self.user_id).await
    }
}

#[derive(async_graphql::InputObject, Serialize, Deserialize)]
pub struct WishNew {
    pub user_id: ObjectId,
    pub aphorism: String,
    pub author: String,
    #[graphql(skip)]
    pub published: bool,
}

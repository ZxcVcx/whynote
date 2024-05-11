use async_graphql::Context;
use bson::oid::ObjectId;

use crate::{
    dbs::mongo::DataSource,
    users::models::{SignInfo, User},
    utils::constants::GqlResult,
};

use crate::users;

#[derive(Default)]
pub struct UserQuery;

#[async_graphql::Object]
impl UserQuery {
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

    async fn default_user(&self, ctx: &Context<'_>) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::default_user(db).await
    }
}

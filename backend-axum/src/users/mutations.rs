use async_graphql::Context;

use crate::{
    dbs::mongo::DataSource,
    users::models::{User, UserNew},
    utils::constants::GqlResult,
};

use super::services;

#[derive(Default)]
pub struct UserMutation;

#[async_graphql::Object]
impl UserMutation {
    async fn user_register(&self, ctx: &Context<'_>, user_new: UserNew) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        self::services::user_register(db, user_new).await
    }

    async fn user_change_password(
        &self,
        ctx: &Context<'_>,
        old_password: String,
        new_password: String,
        token: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        self::services::user_change_password(db, &old_password, &new_password, &token).await
    }

    async fn user_update_profile(
        &self,
        ctx: &Context<'_>,
        user_new: UserNew,
        token: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        self::services::user_update_profile(db, user_new, &token).await
    }
}

use async_graphql::Context;

use crate::{
    dbs::mongo::DataSource,
    users::{
        self,
        models::{User, UserNew},
    },
    utils::constants::GqlResult, wishes::{self, models::{Wish, WishNew}},
};

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn user_register(&self, ctx: &Context<'_>, user_new: UserNew) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::user_register(db, user_new).await
    }

    async fn user_change_password(
        &self,
        ctx: &Context<'_>,
        old_password: String,
        new_password: String,
        token: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::user_change_password(db, &old_password, &new_password, &token).await
    }

    async fn user_update_profile(
        &self,
        ctx: &Context<'_>,
        user_new: UserNew,
        token: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::user_update_profile(db, user_new, &token).await
    }

    // Add new wish
        async fn wish_new(
            &self,
            ctx: &Context<'_>,
            wish_new: WishNew,
        ) -> GqlResult<Wish> {
            let db = ctx.data_unchecked::<DataSource>().db.clone();
            wishes::services::wish_new(db, wish_new).await
        }

    // async fn user_delete(&self, ctx: &Context<'_>, email: String) -> GqlResult<User> {
    //     let db = ctx.data_unchecked::<DataSource>().db.clone();
    //     users::services::user_delete(db, &email).await
    // }
}

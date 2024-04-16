use async_graphql::Context;

use crate::{
    dbs::mongo::DataSource,
    users::{
        self,
        models::{NewUser, User},
    },
    utils::constants::GqlResult,
};

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn new_user(&self, ctx: &Context<'_>, new_user: NewUser) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::new_user(db, new_user).await
    }
}

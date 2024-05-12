use async_graphql::MergedObject;

use crate::{articles, categories, topics, users, wishes};

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    users::queries::UserQuery,
    articles::queries::ArticleQuery,
    categories::queries::CategoryQuery,
    topics::queries::TopicQuery,
    wishes::queries::WishQuery,
);
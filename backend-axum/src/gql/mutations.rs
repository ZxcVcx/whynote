use async_graphql::MergedObject;

use crate::{articles, categories, topics, users, wishes};

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    users::mutations::UserMutation,
    wishes::mutations::WishMutation,
    articles::mutations::ArticleMutation,
    categories::mutations::CategoryMutation,
    topics::mutations::TopicMutation,
);

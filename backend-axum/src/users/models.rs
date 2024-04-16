use bson::oid::ObjectId;
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
    pub cred: String,
}

#[async_graphql::ComplexObject]
impl User {
    pub async fn from(&self) -> String {
        let mut from = String::new();
        from.push_str(&self.username);
        from.push_str(" <");
        from.push_str(&self.email);
        from.push_str(">");
        from
    }
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
#[derive(Serialize, Deserialize, async_graphql::InputObject, Clone)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    #[graphql(skip)]
    pub cred: String,
}

use async_graphql::{Error, ErrorExtensions};
use futures::stream::StreamExt;

use mongodb::Database;

use crate::{users::models::NewUser, users::models::User, utils::constants::GqlResult};

/* pub async fn all_users_two(db: &Database) -> Vec<User> {
    let collection = db.collection("users");
    let cursor = collection.find(None, None).await.unwrap();
    let mut users: Vec<User> = Vec::new();
    for result in cursor {
        match result {
            Ok(document) => {
                let user: User = bson::from_bson(bson::Bson::Document(document)).unwrap();
                users.push(user);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    users
} */

pub async fn all_users(db: Database) -> GqlResult<Vec<User>> {
    let coll = db.collection("users");

    let mut users: Vec<User> = vec![];

    // Query all documents in the collection.
    let mut cursor = coll.find(None, None).await.unwrap();

    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let user = bson::from_bson(bson::Bson::Document(document)).unwrap();
                users.push(user);
            }
            Err(error) => Err(Error::new("6-all-users")
                .extend_with(|_, e| e.set("details", format!("Error to find doc: {}", error))))
            .unwrap(),
        }
    }

    if users.len() > 0 {
        Ok(users)
    } else {
        Err(Error::new("6-all-users").extend_with(|_, e| e.set("details", "No records")))
    }
}

/// get user info by email
pub async fn get_user_by_email(db: Database, email: &str) -> GqlResult<User> {
    let coll = db.collection("users");

    let filter = bson::doc! { "email": email };
    let exist_document = coll.find_one(filter, None).await;

    match exist_document {
        Ok(user_document_exist) => {
            match user_document_exist {
                Some(user_document) => {
                    let user: User = bson::from_bson(bson::Bson::Document(user_document)).unwrap();
                    Ok(user)
                }
                None => Err(Error::new("2-email")
                    .extend_with(|_, e| e.set("details", "Email does not exist"))),
            }
        }
        // Ok(Some(user_document)) => {
        //     let user: User = bson::from_bson(bson::Bson::Document(user_document)).unwrap();
        //     Ok(user)
        // }
        // Ok(None) => Err(Error::new("2-email")
        //     .extend_with(|_, e| e.set("details", "Email does not exist"))),
        Err(_) => {
            Err(Error::new("2-email")
                .extend_with(|_, e| e.set("details", "Error querying MongoDB")))
        }
    }
}

/// add new user
pub async fn new_user(db: Database, new_user: NewUser) -> GqlResult<User> {
    let coll = db.collection("users");

    let mut target_user = new_user.clone();
    target_user.email = target_user.email.to_lowercase();

    if self::get_user_by_email(db.clone(), &target_user.email)
        .await
        .is_ok()
    {
        Err(Error::new("3-new-user").extend_with(|_, e| e.set("details", "Email already exists")))
    } else {
        // new_user.cred = bcrypt::hash(new_user.cred, bcrypt::DEFAULT_COST).unwrap();
        target_user.cred = "P38V7+1Q5sjuKvaZEXnXQqI9SiY6ZMisB8QfUOP91Ao=".to_string();
        let new_user_bson = bson::to_bson(&target_user).unwrap();

        if let bson::Bson::Document(document) = new_user_bson {
            coll.insert_one(document, None)
                .await
                .expect("文档插入 MongoDB 失败");
            self::get_user_by_email(db.clone(), &target_user.email).await
        } else {
            Err(Error::new("3-new-user")
                .extend_with(|_, e| e.set("details", "转换 BSON 对象为 MongoDB 文档时出错")))
        }
    }
}

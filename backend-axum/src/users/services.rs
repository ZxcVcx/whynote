use async_graphql::{Error, ErrorExtensions};
use bson::{doc, oid::ObjectId, to_document, Document};
use futures::stream::StreamExt;

use jsonwebtoken::EncodingKey;
use mongodb::Database;
use regex::Regex;

use crate::{
    users::models::{User, UserNew},
    utils::{
        constants::{GqlResult, CFG},
        cred::{cred_encode, cred_verify, token_data, Claims},
    },
};

use super::models::SignInfo;

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

pub async fn users(db: Database, token: &str) -> GqlResult<Vec<User>> {
    let token_data = token_data(token).await;

    if token_data.is_ok() {
        let coll = db.collection("users");
        let mut users: Vec<User> = vec![];

        // Query all documents in the collection.
        let mut cursor = coll.find(bson::Document::new()).await.unwrap();

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let user = bson::from_bson(bson::Bson::Document(document)).unwrap();
                    users.push(user);
                }
                Err(error) => Err(Error::new("6-all-users").extend_with(|_, e| {
                    e.set("details", format!("Error to find user: {}", error))
                }))?
            }
        }
        Ok(users)
    } else {
        Err(Error::new("No token").extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

pub async fn user_by_id(db: Database, id: ObjectId) -> GqlResult<User> {
    let coll = db.collection::<Document>("users");

    let filter = doc! { "_id": id };
    let user_document = coll.find_one(filter).await.expect("Document not found");

    match user_document {
        Some(user_document) => {
            let user = bson::from_document(user_document)?;
            Ok(user)
        }
        None => Err(Error::new("User not found")
            .extend_with(|err, eev| eev.set("details", err.message.as_str()))),
    }
}

// get user info by username
pub async fn user_by_username(db: Database, username: &str) -> GqlResult<User> {
    let coll = db.collection::<Document>("users");

    let exist_document = coll.find_one(doc! {"username": username}).await;

    if let Ok(user_document_exist) = exist_document {
        if let Some(user_document) = user_document_exist {
            let user: User = bson::from_document(user_document)?;
            Ok(user)
        } else {
            Err(Error::new("Username not found")
                .extend_with(|err, eev| eev.set("details", err.message.as_str())))
        }
    } else {
        Err(Error::new("Error searching mongodb")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

/// get user info by email
pub async fn user_by_email(db: Database, email: &str) -> GqlResult<User> {
    let coll = db.collection("users");

    let filter = bson::doc! { "email": email };
    let exist_document = coll.find_one(filter).await;

    match exist_document {
        Ok(user_document_exist) => {
            match user_document_exist {
                Some(user_document) => {
                    // let user: User = bson::from_bson(bson::Bson::Document(user_document)).unwrap();
                    let user = bson::from_document(user_document)?;
                    Ok(user)
                }
                None => Err(Error::new("2-email")
                    .extend_with(|_, e| e.set("details", "Email does not exist"))),
            }
        }
        Err(_) => Err(Error::new("Error searching mongodb")
            .extend_with(|_, e| e.set("details", "Error querying MongoDB"))),
    }
}

pub async fn user_sign_in(db: Database, signature: &str, password: &str) -> GqlResult<SignInfo> {
    let signature = &signature.to_ascii_lowercase();

    let is_email = Regex::new(r"(@)")?.is_match(signature);

    let user_res = match is_email {
        true => user_by_email(db.clone(), signature).await,
        false => user_by_username(db.clone(), signature).await,
    };

    if let Ok(user) = user_res {
        let is_verified = cred_verify(&user.username, password, &user.cred).await;
        if is_verified {
            // let mut header = jsonwebtoken::Header::default();
            let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
            // header.kid = Some("signing_key".to_owned());
            // header.alg = jsonwebtoken::Algorithm::HS512;

            let jwt_secret = CFG.get("JWT_SECRET").unwrap().as_bytes();
            let claims = Claims::new(user.email.to_owned(), user.username.to_owned());

            let token =
                match jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(jwt_secret))
                {
                    Ok(t) => t,
                    Err(error) => Err(Error::new("Error to encode token")
                        .extend_with(|_, e| e.set("details", format!("{}", error))))?,
                };

            let sign_info = SignInfo {
                email: user.email,
                username: user.username,
                token,
            };
            Ok(sign_info)
        } else {
            Err(Error::new("Invalid credential")
                .extend_with(|err, eev| eev.set("details", err.message.as_str())))
        }
    } else {
        Err(Error::new("User not exist")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

/// add new user
pub async fn user_register(db: Database, user_new: UserNew) -> GqlResult<User> {
    let coll = db.collection("users");

    let mut target_user = user_new.clone();
    target_user.email = target_user.email.to_ascii_lowercase().trim().to_string();
    target_user.username = target_user.username.to_ascii_lowercase().trim().to_string();

    if self::user_by_email(db.clone(), &target_user.email)
        .await
        .is_ok()
    {
        Err(Error::new("3-new-user").extend_with(|_, e| e.set("details", "Email already exists")))
    } else if self::user_by_username(db.clone(), &target_user.username)
        .await
        .is_ok()
    {
        Err(Error::new("3-new-user")
            .extend_with(|_, e| e.set("details", "Username already exists")))
    } else {
        // new_user.cred = bcrypt::hash(new_user.cred, bcrypt::DEFAULT_COST).unwrap();
        target_user.cred = cred_encode(&target_user.username, &target_user.cred).await;
        target_user.banned = false;

        let mut user_new_document = bson::to_document(&target_user)?;
        let now = bson::DateTime::now();
        user_new_document.insert("picture", "/static/favicon.ico");
        user_new_document.insert("created_at", now);
        user_new_document.insert("updated_at", now);

        coll.insert_one(user_new_document.clone())
            .await
            .expect("Error inserting user");

        self::user_by_email(db.clone(), &target_user.email).await
    }
}

pub async fn user_change_password(
    db: Database,
    old_password: &str,
    new_password: &str,
    token: &str,
) -> GqlResult<User> {
    let token_data = token_data(token).await;
    if let Ok(data) = token_data {
        let email = data.claims.email;
        let user_res = user_by_email(db.clone(), &email).await;
        if let Ok(mut user) = user_res {
            let is_verified = cred_verify(&user.username, old_password, &user.cred).await;
            if is_verified {
                user.cred = cred_encode(&user.username, new_password).await;
                let coll = db.collection::<Document>("users");
                let filter = doc! { "_id": &user.id };
                let update = doc! { "$set": { "cred": &user.cred } };
                coll.update_one(filter, update)
                    .await
                    .expect("Error updating user password");
                self::user_by_email(db.clone(), &user.email).await
            } else {
                Err(Error::new("user_change_password")
                    .extend_with(|_, eev| eev.set("details", "Error verifying current passwordd")))
            }
        } else {
            Err(Error::new("User not exist")
                .extend_with(|err, eev| eev.set("details", err.message.as_str())))
        }
    } else {
        Err(Error::new("No token").extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

pub async fn user_update_profile(db: Database, user_new: UserNew, token: &str) -> GqlResult<User> {
    let token_data = token_data(token).await;
    if let Ok(data) = token_data {
        let email = data.claims.email;
        let user_res = self::user_by_email(db.clone(), &email).await;
        if let Ok(mut user) = user_res {
            let coll = db.collection::<Document>("users");
            let filter = doc! { "_id": &user.id };

            user.email = user_new.email.to_ascii_lowercase();
            user.username = user_new.username.to_ascii_lowercase();
            user.nickname = user_new.nickname;
            user.blog_name = user_new.blog_name;
            user.website = user_new.website;
            user.bio = user_new.bio;

            let user_document = to_document(&user)?;
            coll.find_one_and_replace(filter, user_document)
                .await
                .expect("Failed to replae a MongoDB collection!");

            self::user_by_email(db.clone(), &user.email).await
        } else {
            Err(Error::new("User not exist")
                .extend_with(|err, eev| eev.set("details", err.message.as_str())))
        }
    } else {
        Err(Error::new("No token").extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

pub async fn default_user(db: Database) -> GqlResult<User> {
    let coll = db.collection::<Document>("users");

    // sort users by created_at
    // let find_options = mongodb::options::FindOneOptions::builder()
    //     .sort(doc! { "created_at": -1 })
    //     .build();

    let user_document = coll
        .find_one(bson::Document::new())
        .sort(doc! {"created_at": -1 })
        //.find_one(None, find_options)
        .await
        .expect("Document not found");

    match user_document {
        Some(user_document) => {
            let user = bson::from_document(user_document)?;
            Ok(user)
        }
        None => Err(Error::new("Default user not found").extend_with(|_, eev| {
            eev.set(
                "details",
                "Default user not found, May be the database is empty",
            )
        })),
    }
}

use async_graphql::{Error, ErrorExtensions};
use bson::{doc, from_document, to_document, DateTime, Document};
use futures::stream::StreamExt;

use mongodb::Database;

use crate::{users::services::user_by_username, utils::constants::GqlResult};

use super::models::{Wish, WishNew};

// Create new wish
pub async fn wish_new(db: Database, wish_new: WishNew) -> GqlResult<Wish> {
    let coll = db.collection::<Document>("wishes");

    let exist_document = coll
        .find_one(
            doc! {"user_id": &wish_new.user_id, "aphorism": &wish_new.aphorism},
            None,
        )
        .await?;
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let mut wish_new_document = to_document(&wish_new)?;
        let now = DateTime::now();
        wish_new_document.insert("created_at", now);
        wish_new_document.insert("updated_at", now);

        // Insert into a MongoDB collection
        coll.insert_one(wish_new_document, None)
            .await
            .expect("Failed to insert into a MongoDB collection!");
    }

    let wish_document = coll
        .find_one(
            doc! {"user_id": &wish_new.user_id, "aphorism": &wish_new.aphorism},
            None,
        )
        .await
        .expect("Document not found")
        .unwrap();

    let wish: Wish = from_document(wish_document)?;
    Ok(wish)
}

// get all wishes
pub async fn wishes(db: Database, published: i32) -> GqlResult<Vec<Wish>> {
    let mut find_doc = doc! {};
    if published > 0 {
        find_doc.insert("published", true);
    } else if published < 0 {
        find_doc.insert("published", false);
    }
    let coll = db.collection::<Document>("wishes");
    let mut cursor = coll.find(find_doc, None).await?;

    let mut wishes: Vec<Wish> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let wish = from_document(document)?;
                wishes.push(wish);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }
    Ok(wishes)
}

// get random wish
pub async fn random_wish(db: Database, username: &str) -> GqlResult<Wish> {
    let mut find_doc = doc! {"published": true};
    if "".ne(username.trim()) && "-".ne(username.trim()) {
        let user = user_by_username(db.clone(), username).await?;
        find_doc.insert("user_id", &user.id);
    }
    let match_doc = doc! {"$match": find_doc};

    let one_wish = self::one_wish(db.clone(), match_doc).await;
    if one_wish.is_ok() {
        one_wish
    } else {
        self::one_wish(db, doc! {"$match": {"published": true}}).await
    }
}

async fn one_wish(db: Database, match_doc: Document) -> GqlResult<Wish> {
    let coll = db.collection::<Document>("wishes");
    let mut cursor = coll
        .aggregate(vec![doc! {"$sample": {"size": 1}}, match_doc], None)
        .await?;

    if let Some(result) = cursor.next().await {
        let wish = from_document(result?)?;
        Ok(wish)
    } else {
        Err(Error::new("No records")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}
use async_graphql::{Error, ErrorExtensions};
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, to_document, DateTime, Document},
    Database,
};

use crate::{
    users,
    utils::{common::slugify, constants::GqlResult, cred::token_data},
};

use super::models::{Category, CategoryNew, CategoryUser, CategoryUserNew};

pub async fn category_new_by_token(
    db: Database,
    category_new_data: CategoryNew,
    token: &str,
) -> GqlResult<Category> {
    let token_data = token_data(token).await;
    if token_data.is_ok() {
        let token_email = token_data.unwrap().claims.email;
        let user = users::services::user_by_email(db.clone(), &token_email).await?;
        let user_id = user.id;
        let category: Category = category_new(db.clone(), category_new_data).await?;
        let category_user_new_data = CategoryUserNew {
            user_id,
            category_id: category.id,
        };
        category_user_new(db.clone(), category_user_new_data).await?;
        Ok(category)
    } else {
        Err(Error::new("No token").extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

// Create new category
pub async fn category_new(db: Database, mut category_new: CategoryNew) -> GqlResult<Category> {
    let coll = db.collection::<Document>("categories");

    let exist_document = coll
        .find_one(doc! {"name": &category_new.name.to_lowercase()})
        .await?;
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let slug = slugify(&category_new.name).await;
        let uri = format!("/category/{}", &slug);

        category_new.slug = slug;
        category_new.uri = uri;

        let mut category_new_document = to_document(&category_new)?;
        let now = DateTime::now();
        category_new_document.insert("created_at", now);
        category_new_document.insert("updated_at", now);

        // Insert into a MongoDB collection
        coll.insert_one(category_new_document)
            .await
            .expect("Failed to insert into a MongoDB collection!");
    }

    let category_document = coll
        .find_one(doc! {"name": &category_new.name})
        .await
        .expect("Document not found")
        .unwrap();

    let category: Category = from_document(category_document)?;
    Ok(category)
}

// Create new category_user
pub async fn category_user_new(
    db: Database,
    category_user_new: CategoryUserNew,
) -> GqlResult<CategoryUser> {
    let coll = db.collection::<Document>("categories_users");

    let exist_document = coll
        .find_one(doc! {"user_id": &category_user_new.user_id, "category_id": &category_user_new.category_id})
        .await
        .unwrap();
    if let Some(_document) = exist_document {
        // println!("MongoDB document is exist!");
        tracing::info!(
            user_id = &category_user_new.user_id.to_hex(),
            category_id = &category_user_new.category_id.to_hex(),
            "CategoryUser document is exist!"
        );
    } else {
        let category_user_new_document = to_document(&category_user_new)?;
        // Insert into a MongoDB collection
        coll.insert_one(category_user_new_document)
            .await
            .expect("Failed to insert into a MongoDB collection!");
    }

    let category_user_document = coll
        .find_one(doc! {"user_id": &category_user_new.user_id, "category_id": &category_user_new.category_id})
        .await
        .expect("Document not found")
        .unwrap();

    let category_user: CategoryUser = from_document(category_user_document)?;
    Ok(category_user)
}

pub async fn category_delete(
    db: Database,
    category_id: ObjectId,
    token: &str,
) -> GqlResult<Category> {
    let token_data = token_data(token).await;
    if token_data.is_ok() {
        let coll = db.collection::<Document>("categories");

        let category_document = coll
            .find_one(doc! {"_id": category_id})
            .await
            .expect("Document not found")
            .unwrap();

        let category: Category = from_document(category_document)?;

        let articles_published =
            crate::articles::services::articles_by_category_id(db.clone(), category_id, true)
                .await
                .expect("Failed to get articles by category_id");
        let articles_unpublished =
            crate::articles::services::articles_by_category_id(db.clone(), category_id, false)
                .await
                .expect("Failed to get articles by category_id");
        if !articles_unpublished.is_empty() || !articles_published.is_empty() {
            return Err(Error::new("Category has articles")
                .extend_with(|err, eev| eev.set("details", err.message.as_str())));
        }

        coll.delete_one(doc! {"_id": category_id})
            .await
            .expect("Failed to delete a MongoDB collection!");

        let coll_categories_users = db.collection::<Document>("categories_users");
        coll_categories_users
            .delete_many(doc! {"category_id": category_id})
            .await
            .expect("Failed to delete a MongoDB collection!");

        Ok(category)
    } else {
        Err(Error::new("No token").extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

pub async fn category_update(
    db: Database,
    category_id: ObjectId,
    category_new: CategoryNew,
    token: &str,
) -> GqlResult<Category> {
    let token_data = token_data(token).await;
    if token_data.is_ok() {
        let coll = db.collection::<Document>("categories");

        let category_document = coll
            .find_one(doc! {"_id": category_id})
            .await
            .expect("Document not found")
            .unwrap();

        let mut category: Category = from_document(category_document)?;

        let slug = slugify(&category_new.name).await;
        let uri = format!("/category/{}", &slug);

        category.name = category_new.name;
        category.description = category_new.description;
        category.slug = slug;
        category.uri = uri;
        category.updated_at = DateTime::now();

        let category_document = to_document(&category)?;
        coll.update_one(doc! {"_id": category_id}, doc! {"$set": category_document})
            .await
            .expect("Failed to update a MongoDB collection!");

        Ok(category)
    } else {
        Err(Error::new("No token").extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

// get all categories
pub async fn categories(db: Database) -> GqlResult<Vec<Category>> {
    let coll = db.collection::<Document>("categories");

    // Query all documents in the collection.
    // let find_options = FindOptions::builder().sort(doc! {"quotes": -1}).build();
    let mut cursor = coll.find(doc! {}).sort(doc! {"quotes": -1}).await.unwrap();
    // let mut cursor = coll.find(None, find_options).await.unwrap();

    // Iterate over the results of the cursor.
    let mut categories: Vec<Category> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let category = from_document(document)?;
                categories.push(category);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(categories)
}

// get all categories by user_id
pub async fn categories_by_user_id(db: Database, user_id: ObjectId) -> GqlResult<Vec<Category>> {
    let categories_users = self::categories_users_by_user_id(db.clone(), user_id).await;

    let mut category_ids: Vec<ObjectId> = vec![];
    for category_user in categories_users {
        category_ids.push(category_user.category_id);
    }

    let coll_categories = db.collection::<Document>("categories");
    let mut cursor_categories = coll_categories
        .find(doc! {"_id": {"$in": category_ids}})
        .await?;

    let mut categories: Vec<Category> = vec![];
    while let Some(result) = cursor_categories.next().await {
        match result {
            Ok(document) => {
                let category: Category = from_document(document)?;
                categories.push(category);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(categories)
}

// get all categories by username
pub async fn categories_by_username(db: Database, username: &str) -> GqlResult<Vec<Category>> {
    let user = crate::users::services::user_by_username(db.clone(), username).await?;
    self::categories_by_user_id(db, user.id).await
}

// get category by its id
pub async fn category_by_id(db: Database, id: ObjectId) -> GqlResult<Category> {
    let coll = db.collection::<Document>("categories");

    let category_document = coll
        .find_one(doc! {"_id": id})
        .await
        .expect("Document not found")
        .unwrap();

    let category: Category = from_document(category_document)?;
    Ok(category)
}

// get category by its slug
pub async fn category_by_slug(db: Database, slug: &str) -> GqlResult<Category> {
    let coll = db.collection::<Document>("categories");

    let category_document = coll
        .find_one(doc! {"slug": slug.to_lowercase()})
        .await
        .expect("Document not found")
        .unwrap();

    let category: Category = from_document(category_document)?;
    Ok(category)
}

// get all CategoryUser list by user_id
async fn categories_users_by_user_id(db: Database, user_id: ObjectId) -> Vec<CategoryUser> {
    let coll_categories_users = db.collection::<Document>("categories_users");
    let mut cursor_categories_users = coll_categories_users
        .find(doc! {"user_id": user_id})
        .await
        .unwrap();

    let mut categories_users: Vec<CategoryUser> = vec![];
    // Iterate over the results of the cursor.
    while let Some(result) = cursor_categories_users.next().await {
        match result {
            Ok(document) => {
                let category_user: CategoryUser = from_document(document).unwrap();
                categories_users.push(category_user);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    categories_users
}

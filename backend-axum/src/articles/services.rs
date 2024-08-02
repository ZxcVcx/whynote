// use std::result;

use async_graphql::{Error, ErrorExtensions};
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, to_document, DateTime, Document},
    Database,
};

use crate::utils::{common::slugify, constants::GqlResult, cred::token_data};
use crate::{topics::models::TopicArticle, users};

// use crate::topics::models::TopicArticle;
use super::models::{Article, ArticleNew};

pub async fn article_delete(db: Database, article_id: ObjectId, token: &str) -> GqlResult<Article> {
    let coll = db.collection::<Document>("articles");

    let article_document = coll
        .find_one(doc! {"_id": article_id})
        .await
        .expect("Document not found")
        .unwrap();

    let article: Article = from_document(article_document)?;

    let token_data = token_data(token).await;
    if token_data.is_err() {
        return Err("Token is invalid!".into());
    }
    let token_email = token_data.unwrap().claims.email;
    let user = users::services::user_by_email(db.clone(), &token_email).await?;
    let user_id = user.id;
    if user_id.eq(&article.user_id) {
        coll.delete_one(doc! {"_id": article_id})
            .await
            .expect("Failed to delete a MongoDB document!");
    }
    Ok(article)
}

pub async fn article_update(
    db: Database,
    article_id: ObjectId,
    mut article_new: ArticleNew,
    token: &str,
) -> GqlResult<Article> {
    let coll = db.collection::<Document>("articles");

    let article_document = coll
        .find_one(doc! {"_id": article_id})
        .await
        .expect("Document not found")
        .unwrap();

    let article: Article = from_document(article_document)?;

    let token_data = token_data(token).await;
    if token_data.is_err() {
        return Err("Token is invalid!".into());
    }
    let token_email = token_data.unwrap().claims.email;
    let user = users::services::user_by_email(db.clone(), &token_email).await?;
    let user_id = user.id;
    if user_id.eq(&article.user_id) {
        let slug = slugify(&article_new.subject).await;

        let user = users::services::user_by_id(db.clone(), article_new.user_id).await?;
        let uri = format!("/{}/{}", &user.username, &slug);

        article_new.slug = slug;
        article_new.uri = uri;
        // article_new.published = false; // false;
        // article_new.top = false; // false;
        // article_new.recommended = false; // false;

        let mut article_new_document = to_document(&article_new)?;
        let now = DateTime::now();
        article_new_document.insert("updated_at", now);

        // Insert into a MongoDB collection
        coll.update_one(
            doc! {"_id": article_id},
            doc! {"$set": article_new_document},
        )
        .await
        .expect("Failed to update a MongoDB document!");
    }
    self::article_by_slug(db, slugify(&article_new.subject).await.as_str()).await
}

pub async fn article_new(db: Database, mut article_new: ArticleNew) -> GqlResult<Article> {
    let coll = db.collection::<Document>("articles");

    let exist_document = coll
        .find_one(doc! {"user_id": article_new.user_id,  "subject": &article_new.subject})
        .await
        .unwrap();
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let slug = slugify(&article_new.subject).await;

        let user = users::services::user_by_id(db.clone(), article_new.user_id).await?;
        let uri = format!("/{}/{}", &user.username, &slug);

        article_new.slug = slug;
        article_new.uri = uri;
        // article_new.published = article_new.published; // false;
        // article_new.top = article_new.top; // false;
        // article_new.recommended = article_new.recommended; // false;

        let mut article_new_document = to_document(&article_new)?;
        let now = DateTime::now();
        article_new_document.insert("created_at", now);
        article_new_document.insert("updated_at", now);

        // Insert into a MongoDB collection
        coll.insert_one(article_new_document)
            .await
            .expect("Failed to insert into a MongoDB collection!");
    }

    let article_document = coll
        .find_one(doc! {"user_id": article_new.user_id,  "subject": &article_new.subject})
        .await
        .expect("Document not found")
        .unwrap();

    let article: Article = from_document(article_document)?;
    Ok(article)
}

pub async fn article_by_username_and_slug(
    db: Database,
    username: &str,
    slug: &str,
) -> GqlResult<Article> {
    let coll = db.collection::<Document>("articles");

    let user = users::services::user_by_username(db.clone(), username).await?;
    let article_document = coll
        .find_one(doc! {"user_id": user.id, "slug": slug.to_lowercase()})
        .await
        .expect("Document not found")
        .unwrap();

    let article: Article = from_document(article_document)?;
    Ok(article)
}

pub async fn article_by_slug(db: Database, slug: &str) -> GqlResult<Article> {
    let coll = db.collection::<Document>("articles");

    // let user = users::services::user_by_username(db.clone(), username).await?;
    let article_document = coll
        .find_one(doc! {"slug": slug.to_lowercase()})
        .await
        .expect("Document not found")
        .unwrap();

    let article: Article = from_document(article_document)?;
    Ok(article)
}

pub async fn article_by_id(db: Database, article_id: ObjectId) -> GqlResult<Article> {
    let coll = db.collection::<Document>("articles");

    let article_document = coll
        .find_one(doc! {"_id": article_id})
        .await
        .expect("Document not found")
        .unwrap();

    let article: Article = from_document(article_document)?;
    Ok(article)
}

pub async fn articles(db: Database, published: bool) -> GqlResult<Vec<Article>> {
    let mut find_doc = doc! {};
    find_doc.insert("published", published);
    let coll = db.collection::<Document>("articles");

    // let find_options = FindOptions::builder().sort(doc! {"updated_at": -1}).build();
    let mut cursor = coll
        .find(find_doc)
        .sort(doc! {"updated_at": -1})
        .await
        .unwrap();
    // let mut cursor = coll.find(find_doc, find_options).await.unwrap();

    let mut articles: Vec<Article> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article = from_document(document)?;
                articles.push(article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(articles)
}

pub async fn articles_in_position(
    db: Database,
    username: &str,
    position: &str,
    limit: i64,
) -> GqlResult<Vec<Article>> {
    let mut find_doc = doc! {"published": true};
    if "".ne(username.trim()) && "-".ne(username.trim()) {
        let user = users::services::user_by_username(db.clone(), username).await?;
        find_doc.insert("user_id", user.id);
    }
    if "top".eq(position.trim()) {
        find_doc.insert("top", true);
    }
    if "recommended".eq(position.trim()) {
        find_doc.insert("recommended", true);
    }

    let coll = db.collection::<Document>("articles");

    // let find_options = FindOptions::builder()
    //     .sort(doc! {"updated_at": -1})
    //     .limit(limit)
    //     .build();
    // let mut cursor = coll.find(find_doc, find_options).await?;
    let mut cursor = coll
        .find(find_doc)
        .sort(doc! {"updated_at": -1})
        .limit(limit)
        .await?;

    let mut articles: Vec<Article> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article = from_document(document)?;
                articles.push(article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(articles)
}

pub async fn articles_by_user_id(
    db: Database,
    user_id: ObjectId,
    published: bool,
) -> GqlResult<Vec<Article>> {
    let coll = db.collection::<Document>("articles");

    let mut find_doc = doc! {"user_id": user_id};
    find_doc.insert("published", published);
    // let find_options = FindOptions::builder().sort(doc! {"updated_at": -1}).build();

    // let mut cursor = coll.find(find_doc, find_options).await?;

    let mut cursor = coll.find(find_doc).sort(doc! {"updated_at": -1}).await?;

    let mut articles: Vec<Article> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article = from_document(document)?;
                articles.push(article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(articles)
}

pub async fn articles_by_username(
    db: Database,
    username: &str,
    published: bool,
) -> GqlResult<Vec<Article>> {
    let user = users::services::user_by_username(db.clone(), username).await?;
    self::articles_by_user_id(db, user.id, published).await
}

// Get all articles by category_id
pub async fn articles_by_category_id(
    db: Database,
    category_id: ObjectId,
    published: bool,
) -> GqlResult<Vec<Article>> {
    let mut find_doc = doc! {"category_id": category_id};
    // if published > 0 {
    //     find_doc.insert("published", true);
    // } else if published < 0 {
    //     find_doc.insert("published", false);
    // }
    find_doc.insert("published", published);
    // let find_options = FindOptions::builder().sort(doc! {"updated_at": -1}).build();

    let coll = db.collection::<Document>("articles");
    // let mut cursor = coll.find(find_doc, find_options).await?;
    let mut cursor = coll.find(find_doc).sort(doc! {"updated_at": -1}).await?;
    let mut articles: Vec<Article> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article = from_document(document)?;
                articles.push(article);
            }
            Err(error) => Err(Error::new("2-articles-by-articles-id").extend_with(|_, e| {
                e.set("details", format!("Error to find articles: {}", error))
            }))?
        }
    }

    Ok(articles)
}

// Get all articles by topic_id
pub async fn articles_by_topic_id(
    db: Database,
    topic_id: ObjectId,
    published: bool,
) -> GqlResult<Vec<Article>> {
    let topics_articles = topics_articles_by_topic_id(db.clone(), topic_id).await;

    let mut article_ids = vec![];
    for topic_article in topics_articles {
        article_ids.push(topic_article.article_id);
    }
    article_ids.sort();
    article_ids.dedup();

    let mut find_doc = doc! {"_id": {"$in": article_ids}};
    // if published > 0 {
    //     find_doc.insert("published", true);
    // } else if published < 0 {
    //     find_doc.insert("published", false);
    // }
    find_doc.insert("published", published);
    // let find_options = FindOptions::builder().sort(doc! {"updated_at": -1}).build();

    let coll = db.collection::<Document>("articles");
    // let mut cursor = coll.find(find_doc, find_options).await?;
    let mut cursor = coll.find(find_doc).sort(doc! {"updated_at": -1}).await?;

    let mut articles: Vec<Article> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article = from_document(document)?;
                articles.push(article);
            }
            Err(error) => Err(Error::new("2-articles-by-topic-id").extend_with(|_, e| {
                e.set("details", format!("Error to find articles: {}", error))
            }))?
        }
    }

    Ok(articles)
}

// get all TopicArticle list by topic_id
async fn topics_articles_by_topic_id(db: Database, topic_id: ObjectId) -> Vec<TopicArticle> {
    let coll_topics_articles = db.collection::<Document>("topics_articles");
    let mut cursor_topics_articles = coll_topics_articles
        .find(doc! {"topic_id": topic_id})
        .await
        .unwrap();

    let mut topics_articles: Vec<TopicArticle> = vec![];
    // Iterate over the results of the cursor.
    while let Some(result) = cursor_topics_articles.next().await {
        match result {
            Ok(document) => {
                let topic_article: TopicArticle = from_document(document).unwrap();
                topics_articles.push(topic_article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    topics_articles
}

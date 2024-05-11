use graphql_client::GraphQLQuery;
use serde_json::{json, Value};

use crate::types::{init::ArticleNewType, init::NewUser};

use super::use_query::FetchError;
type ObjectId = String;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/init.graphql",
    response_derives = "Debug, Clone"
)]
struct RegisterData;

async fn fetch_register_data(
    email: &str,
    username: &str,
    nickname: &str,
    cred: &str,
    blog_name: &str,
    website: &str,
    bio: &str,
) -> Result<Value, FetchError> {
    let veriables = register_data::Variables {
        email: email.to_string(),
        username: username.to_string(),
        nickname: nickname.to_string(),
        cred: cred.to_string(),
        blog_name: blog_name.to_string(),
        website: website.to_string(),
        bio: bio.to_string(),
    };
    let query_body = RegisterData::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;

    Ok(data)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/init.graphql",
    response_derives = "Debug, Clone"
)]
struct UserTokenData;

async fn fetch_user_token_data(email: &str, password: &str) -> Result<Value, FetchError> {
    let veriables = user_token_data::Variables {
        signature: email.to_string(),
        password: password.to_string(),
    };
    let query_body = UserTokenData::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;

    Ok(data)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/init.graphql",
    response_derives = "Debug, Clone"
)]
struct CategoryNewData;

async fn fetch_category_new_data(name: &str, description: &str) -> Result<Value, FetchError> {
    let veriables = category_new_data::Variables {
        name: name.to_string(),
        description: description.to_string(),
    };
    let query_body = CategoryNewData::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;

    Ok(data)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/init.graphql",
    response_derives = "Debug, Clone"
)]
struct CategoryUserNewData;

async fn fetch_category_user_new_data(
    user_id: &str,
    category_id: &str,
) -> Result<Value, FetchError> {
    let veriables = category_user_new_data::Variables {
        user_id: user_id.to_string(),
        category_id: category_id.to_string(),
    };
    let query_body = CategoryUserNewData::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;
    Ok(data)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/init.graphql",
    response_derives = "Debug, Clone"
)]
struct TopicsNewData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutation/init.graphql",
    response_derives = "Debug, Clone"
)]
struct ArticleNewData;

async fn fetch_article_new_data(
    user_id: &str,
    category_id: &str,
    subject: &str,
    summary: &str,
    content: &str,
    published: bool,
    top: bool,
    recommended: bool,
) -> Result<Value, FetchError> {
    let veriables = article_new_data::Variables {
        user_id: user_id.to_string(),
        category_id: category_id.to_string(),
        subject: subject.to_string(),
        summary: summary.to_string(),
        content: content.to_string(),
        published,
        top,
        recommended,
    };
    let query_body = ArticleNewData::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;
    Ok(data)
}

pub async fn fetch_new_user_token_data_typed(new_user: &NewUser) -> Result<Value, FetchError> {
    fetch_new_user_token_data(
        new_user.email.as_str(),
        new_user.username.as_str(),
        new_user.nickname.as_str(),
        new_user.password.as_str(),
        new_user.blog_name.as_str(),
        new_user.elsewhere.as_str(),
        new_user.blog_description.as_str(),
    )
    .await
}

pub async fn fetch_new_user_token_data(
    email: &str,
    username: &str,
    nickname: &str,
    password: &str,
    blog_name: &str,
    website: &str,
    bio: &str,
) -> Result<Value, FetchError> {
    match fetch_register_data(email, username, nickname, password, blog_name, website, bio).await {
        Ok(register_data) => {
            let user_id = register_data["userRegister"]["id"].as_str().unwrap();
            match fetch_user_token_data(email, password).await {
                Ok(user_token_data) => {
                    let token = user_token_data["userSignIn"]["token"].as_str().unwrap();
                    let email = user_token_data["userSignIn"]["email"].as_str().unwrap();
                    let username = user_token_data["userSignIn"]["username"].as_str().unwrap();
                    let mut data = json!({});
                    data["userNewToken"] = json!({
                        "id": user_id,
                        "email": email,
                        "username": username,
                        "token": token
                    });
                    Ok(data)
                }
                Err(e) => Err(e),
            }
        }

        // Ok(register_data) => match fetch_user_token_data(email, password).await {
        //     Ok(user_token_data) => Ok(user_token_data),
        //     Err(e) => Err(e),
        // },
        Err(e) => Err(e),
    }
}

// 如果需要快速随机访问，请使用 HashMap。
// 如果需要键的顺序或顺序迭代，请使用 BTreeMap。
// 对于较小或不经常变动的数据集，可以简单使用 Vec 的元组。

pub async fn fetch_new_categories_data(
    categories: Vec<(&str, &str)>,
    user_id: &str,
) -> Result<Value, FetchError> {
    let mut data = json!({});
    for category in categories {
        match fetch_category_new_data(category.0, category.1).await {
            Ok(category_data) => {
                // data[category.0] = category_data;
                let category_id = category_data["categoryNew"]["id"].as_str().unwrap();
                match fetch_category_user_new_data(user_id, category_id).await {
                    Ok(category_user_data) => {
                        // data[category.0] = category_user_data;
                        data[category.0] = category_user_data;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(data)
}

pub async fn fetch_topics_new_data(topic_names: &str) -> Result<Value, FetchError> {
    let veriables = topics_new_data::Variables {
        topic_names: topic_names.to_string(),
    };
    let query_body = TopicsNewData::build_query(veriables);
    let query = json!(query_body);
    let data = super::use_query::fetch_gql_data(query).await?;
    Ok(data)
}

pub async fn fetch_new_articles_data(
    user_id: &str,
    articles_new: Vec<ArticleNewType>,
) -> Result<Value, FetchError> {
    let mut data = json!({});
    let mut article_data_vec = vec![];
    for article in articles_new {
        match fetch_article_new_data(
            user_id,
            article.category_id.as_str(),
            article.subject.as_str(),
            article.summary.as_str(),
            article.content.as_str(),
            article.published,
            article.top,
            article.recommended,
        )
        .await
        {
            Ok(article_data) => {
                // data[article.subject.as_str()] = article_data;
                article_data_vec.push(article_data["articleNew"].clone());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    data["articlesNew"] = Value::Array(article_data_vec);
    Ok(data)
}

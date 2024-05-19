// use std::clone;

use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew::{function_component, html, Html};
use yew_hooks::prelude::*;
// use yew_router::prelude::*;

use crate::components::manage_container::editor::Editor;
use crate::components::top_container::TopContainer;
// use crate::services::article::fetch_article_data_by_id;
use crate::services::use_query::FetchError;
use crate::services::user::fetch_user_data_by_email;
use crate::utils::storage::get_pair_value;

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    nickname: String,
    #[serde(rename = "blogName")]
    blog_name: String,
}

#[derive(Serialize, Deserialize)]
struct Category {
    id: String,
    name: String,
    uri: String,
}

#[derive(Serialize, Deserialize)]
struct Topics {
    id: String,
    name: String,
    uri: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    id: String,
    #[serde(rename = "userId")]
    user_id: String,
    subject: String,
    #[serde(rename = "categoryId")]
    category_id: String,
    summary: String,
    content: String,
    #[serde(rename = "contentHtml")]
    content_html: String,
    published: bool,
    top: bool,
    recommended: bool,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    user: User,
    category: Category,
    topics: Vec<Topics>,
}

#[function_component]
pub fn NewEditorPage() -> Html {
    let email: Result<String, FetchError> = match get_pair_value("email") {
        Some(email) => Ok(email),
        None => Err(FetchError::from(JsValue::from_str("email not found"))),
    };
    if let Some(error) = email.err() {
        return html! {error};
    }
    let logged_in_user_data =
        use_async(async move { fetch_user_data_by_email(get_pair_value("email").unwrap()).await });

    let cloned_logged_in_user_data = logged_in_user_data.clone();

    use_effect_with((), move |_| {
        cloned_logged_in_user_data.run();
        || ()
    });

    html! {
        <>
            {
                if logged_in_user_data.loading {
                    html! {
                        <div class="container py-4 position-fixed">
                            <div class="spinner-border text-primary" role="status">
                                <span class="visually-hidden">{ "Loading..." }</span>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            {
                if let Some(error) = &logged_in_user_data.error {
                    html! {error}
                } else {
                    html! {}
                }
            }
            {
                if let Some(user_data) = &logged_in_user_data.data {
                    let user_data = user_data.clone();
                    let user_id = user_data["userByEmail"]["id"].as_str().unwrap();


                    let empty_article = Article {
                        id: "".to_string(),
                        user_id: user_id.to_string(),
                        subject: "New Article's Subject".to_string(),
                        category_id: "".to_string(),
                        summary: "New article's summary".to_string(),
                        content: "**Hello World**".to_string(),
                        content_html: "".to_string(),
                        published: false,
                        top: false,
                        recommended: false,
                        created_at: "".to_string(),
                        updated_at: "".to_string(),
                        user: User {
                            id: user_id.to_string(),
                            username: "".to_string(),
                            nickname: "".to_string(),
                            blog_name: "".to_string(),
                        },
                        category: Category {
                            id: "".to_string(),
                            name: "".to_string(),
                            uri: "".to_string(),
                        },
                        topics: vec![],
                    };

                    let empty_article = serde_json::to_value(&empty_article).unwrap();
                    // let empty_article = JsValue::from_str(&empty_article);
                    html! {
                        <>
                            <TopContainer />
                            <Editor article={empty_article} />
                        </>
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}

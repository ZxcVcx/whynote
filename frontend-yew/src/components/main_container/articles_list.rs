use serde_json::Value;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{app::MainRoute, utils::common::format_date};


#[derive(PartialEq, Properties)]
pub struct ArticlesListProps {
    pub articles_list: Vec<Value>,
    pub description: Value,
}

#[function_component]
pub fn ArticlesList(props: &ArticlesListProps) -> Html {
    let ArticlesListProps { articles_list, description } = props;
    let navigator = use_navigator().expect("Navigator should be available");
    html! {
        <div class="col-md-8 articles-list">
            <h3 class="pb-4 mb-4 fst-italic border-bottom">
                { description.as_str().unwrap() }
            </h3>
            {
                articles_list.into_iter().map(|article_card| {
                    let subject = article_card.get("subject").unwrap().as_str().unwrap();
                    let summary = article_card.get("summary").unwrap().as_str().unwrap();
                    let updated_at = format_date(article_card.get("updatedAt").unwrap(), "%b %d, %Y").unwrap();
                    let slug = article_card.get("slug").unwrap().as_str().unwrap().to_string();
                    let category = article_card.get("category").unwrap().get("name").unwrap().as_str().unwrap();
                    let user = article_card.get("user").unwrap().as_object().unwrap();
                    let nickname = user.get("nickname").unwrap().as_str().unwrap();
                    let username = user.get("username").unwrap().as_str().unwrap();

                    let on_username_click = {
                        let navigator = navigator.clone();
                        let username = username.to_string();
                        Callback::from(move |_| {
                            navigator.push(&MainRoute::UserPage { username: username.clone() });
                        })
                    };

                    let on_content_click = {
                        let navigator = navigator.clone();
                        Callback::from(move |_| {
                            navigator.push(&MainRoute::ArticlePage {slug: slug.clone() });
                        })
                    };

                    // let avatar = user.get("picture").unwrap().as_str().unwrap();
                    // let topics = article_card.get("topics").unwrap().as_array().unwrap();
                    html! {
                        <div key={username} class="article-card">
                            <div class="row g-0">
                                <div class="col-md-10">
                                    <div class="card-body">
                                        <div class="d-flex justify-content-between align-items-center mb-2">
                                            <span class="text-muted">{format!{"{} by ", updated_at}}
                                            <a onclick={on_username_click} style="color: blue; cursor: pointer;">{nickname} </a>
                                            </span>
                                        </div>
                                        <div onclick={on_content_click} style="cursor: pointer;">
                                            <h5 class="article-card-title">{subject}</h5>
                                            <p class="article-card-text">{summary}</p>
                                            <p class="badge bg-secondary">{category}</p>
                                            <p class="article-read-time">{"2 min read"}</p>
                                        </div>
                                    </div>
                                </div>
                                // <div class="col-md-2">
                                //     <img src="river-crossing.jpg" class="img-fluid rounded-end card-img-right" alt="...">
                                // </div>
                            </div>
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
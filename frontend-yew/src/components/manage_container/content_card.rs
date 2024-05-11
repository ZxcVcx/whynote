use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
// use wasm_bindgen_futures::spawn_local;
use crate::{
    app::{MainRoute, ManageRoute},
    services::article::delete_article_data,
    utils::{
        common::{format_date, shorter_string},
        storage::get_pair_value,
    },
};
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ContentCardProps {
    pub contents: Vec<Value>,
    // pub article_id: String,
    pub published: bool,
}

#[function_component]
pub fn ContentCard(props: &ContentCardProps) -> Html {
    let ContentCardProps {
        contents,
        published,
    } = props;
    let navigator = use_navigator().unwrap();

    // let on_delete_click = {
    //     let navigator = navigator.clone();
    //     let article_id = article_id.clone();
    //     Callback::from(move |_| {
    //         spawn_local(async move {
    //             let data = delete_article_data(, get_pair_value("jwt").unwrap_or("".to_string())).await;
    //             match data {
    //                 Ok(_) => {
    //                     // web_sys::window().unwrap().alert_with_message("Delete successfully!");
    //                     navigator.push(&ManageRoute::Profile);
    //                 }
    //                 Err(_) => {
    //                     // web_sys::window().unwrap().alert_with_message("Delete failed!");
    //                 }
    //             }
    //         }
    //         )
    //     })
    // };
    html! {
        {
            for contents.iter().map(|craft| {
                let subject = craft["subject"].as_str().unwrap();
                let slug = craft["slug"].as_str().unwrap().to_string();
                let summary = craft["summary"].as_str().unwrap();
                let updated_at = format_date(&craft["updatedAt"], "%b %d, %Y").unwrap();
                let category = craft["category"]["name"].as_str().unwrap();
                let article_id = craft["id"].as_str().unwrap().to_string();
                let on_content_click = {
                    let navigator = navigator.clone();
                    Callback::from(move |_| {
                        navigator.push(&MainRoute::ArticlePage {slug: slug.clone() });
                    })
                };
                let on_delete_click = {
                    // let navigator = navigator.clone();
                    let article_id = article_id.clone();
                    Callback::from(move |_| {
                        // let navigator = navigator.clone();
                        let article_id = article_id.clone();
                        spawn_local(async move {
                            let _data = delete_article_data(article_id.clone(), get_pair_value("jwt").unwrap_or("".to_string())).await;
                            web_sys::window().unwrap().location().reload().unwrap();
                            // match data {
                            //     Ok(_) => {
                            //         // web_sys::window().unwrap().alert_with_message("Delete successfully!");
                            //         // navigator.push(&ManageRoute::Profile);
                            //     }
                            //     Err(_) => {
                            //         // navigator.push(&ManageRoute::Profile);
                            //     }
                            // }

                        }
                        )
                    })
                };
                html! {
                    <div class="article-card">
                        <div class="row g-0">
                            <div class="col-md-10">
                                <div class="article-card-body">
                                    <div class="d-flex justify-content-between align-items-center mb-2">
                                        <h6 onclick={on_content_click} class="card-title">{shorter_string(subject, 32)}</h6>
                                    </div>
                                    // <div onclick={on_content_click} style="cursor: pointer;">
                                    <div>

                                        // <h6 onclick={on_content_click} class="card-title">{subject}</h6>
                                        <p class="article-card-text">{shorter_string(summary, 110)}</p>
                                        // <br />
                                        <span class="text-muted">{format!{"Updated at {}", updated_at}}
                                        </span>
                                        <br />
                                        <p class="badge bg-secondary">{category}</p>

                                    </div>
                                </div>
                            </div>
                            <div class="col-md-2">
                                <div class="dropdown d-grid gap-2 d-md-flex justify-content-md-end">
                                    <p class="btn btn-light" type="button" data-bs-toggle="dropdown" aria-expanded="false">{". . ."}</p>
                                    <ul class="dropdown-menu">
                                        {

                                            if published.clone() {
                                                html! {}
                                            } else {
                                                // let on_publish_click = {
                                                //     let navigator = navigator.clone();
                                                //     Callback::from(move |_| {
                                                //         // navigator.push(&MainRoute::PublishPage {slug: slug.clone() });
                                                //     })
                                                // };
                                                html! {
                                                    <li>
                                                        <a class="dropdown-item" href="#">{"Publish"}</a>
                                                    </li>
                                                }
                                            }
                                        }
                                        <li>
                                            <Link<ManageRoute> to={ManageRoute::Editor {id: article_id.clone()} } classes="dropdown-item">{"Edit"}</Link<ManageRoute>>
                                        </li>
                                        <li><a class="dropdown-item"
                                        onclick={on_delete_click}
                                        >
                                        {"Delete"}</a></li>
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            })
        }
    }
}

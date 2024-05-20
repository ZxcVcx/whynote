use yew::prelude::*;

use crate::utils::common::is_logged_in;
use serde_json::Value;
use yew_router::hooks::use_navigator;

use crate::{app::ManageRoute, utils::common::format_date};

#[derive(PartialEq, Properties)]
pub struct ArticleProps {
    pub article: Value,
}

#[function_component]
pub fn Article(props: &ArticleProps) -> Html {
    let ArticleProps { article } = props;
    let updated_at = format_date(article.get("updatedAt").unwrap(), "%b %d, %Y").unwrap();
    let content_html = article.get("contentHtml").unwrap().as_str().unwrap();
    let content_html_section = gloo_utils::document().create_element("section").unwrap();
    content_html_section.set_inner_html(content_html);
    let content_html_node = Html::VRef(content_html_section.into());
    let navigator = use_navigator().unwrap();

    let on_edit_click = {
        let navigator = navigator.clone();
        let article = article.clone();
        let id = article["id"].as_str().unwrap().to_string();
        Callback::from(move |_| navigator.push(&ManageRoute::Editor { id: id.clone() }))
    };
    html! {
        <>
        <article class="blog-post markdown-body">
                <h1 class="display-5 link-body-emphasis mb-1">{ article["subject"].as_str().unwrap() }</h1>
            <span class="d-flex">
                <p class="blog-post-meta me-auto">{ format!("{} by {}", updated_at, article["user"]["username"].as_str().unwrap()) }</p>
                {
                    if is_logged_in() {
                        html! {<button type="button" class="btn btn-outline-secondary" onclick={on_edit_click} >{"edit"} </button>}
                    } else {
                        html! {}
                    }
                }
            </span>
            { content_html_node }
            <hr />
        </article>
        </>
    }
}

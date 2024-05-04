use yew::prelude::*;

use serde_json::Value;

use crate::utils::common::format_date;

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
    html! {
        <article class="blog-post markdown-body">
            <h1 class="display-5 link-body-emphasis mb-1">{ article["subject"].as_str().unwrap() }</h1>
            <p class="blog-post-meta">{ format!("{} by {}", updated_at, article["user"]["username"].as_str().unwrap()) }</p>
            { content_html_node }
            // <hr />
        </article>
    }
}

use yew::prelude::*;

use serde_json::Value;

use crate::utils::common::format_date;

// #[derive(PartialEq, Properties)]
// pub struct ArticleProps {
//     pub title: AttrValue,
//     pub content_html: AttrValue,
//     pub url: AttrValue,
//     pub created_at: AttrValue,
//     pub author: AttrValue,
//     pub content: AttrValue,
// }

// #[function_component(Article)]
// pub fn article(props: &ArticleProps) -> Html {
//     let title = props.title.clone();
//     let created_at = props.created_at.clone();
//     // let url = props.url.clone();
//     let author = props.author.clone();
//     let content_html = props.content_html.clone();
//     html! {
//         <article class="blog-post">
//             <h2 class="display-5 link-body-emphasis mb-1">{ &title }</h2>
//             <p class="blog-post-meta">{created_at}{" by "}<a href="#">{author}</a></p>
//             {content_html}
//         </article>
//     }

// <article class="blog-post">
//     <h2 class="display-5 link-body-emphasis mb-1">{ article["subject"].as_str().unwrap() }</h2>
//     <p class="blog-post-meta">{ format!("{} by {}", updated_at, article["user"]["username"].as_str().unwrap()) }</p>
//     { article["contentHtml"].as_str().unwrap() }
//     <hr />
// </article>
// }

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
        <article class="blog-post">
            <h2 class="display-5 link-body-emphasis mb-1">{ article["subject"].as_str().unwrap() }</h2>
            <p class="blog-post-meta">{ format!("{} by {}", updated_at, article["user"]["username"].as_str().unwrap()) }</p>
            { content_html_node }
            // <hr />
        </article>
    }
}

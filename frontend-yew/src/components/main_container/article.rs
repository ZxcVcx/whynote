use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ArticleProps {
    pub title: AttrValue,
    pub content_html: AttrValue,
    pub url: AttrValue,
    pub created_at: AttrValue,
    pub author: AttrValue,
    pub content: AttrValue,
}

#[function_component(Article)]
pub fn article(props: &ArticleProps) -> Html {
    let title = props.title.clone();
    let created_at = props.created_at.clone();
    // let url = props.url.clone();
    let author = props.author.clone();
    let content_html = props.content_html.clone();
    html! {
        <article class="blog-post">
            <h2 class="display-5 link-body-emphasis mb-1">{ &title }</h2>
            <p class="blog-post-meta">{created_at}{" by "}<a href="#">{author}</a></p>
            {content_html}
        </article>
    }
}
use serde_json::Value;
use yew::prelude::*;

use super::article::Article;

#[derive(PartialEq, Properties)]
pub struct ArticlesProps {
    pub articles: Vec<Value>,
}

#[function_component]
pub fn Articles(props: &ArticlesProps) -> Html {
    let article_vec = props.articles.clone();
    html! {
        // <h3 class="pb-4 mb-4 fst-italic border-bottom">
        //     { "From the Firehose" }
        // </h3>
        <div class="col-md-8">
            <h3 class="pb-4 mb-4 fst-italic border-bottom">
                { "From Nathan Wang" }
            </h3>
            {
                article_vec.into_iter().map(|article| {
                    html! {
                        <Article article={article} />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}

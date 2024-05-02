use serde_json::Value;
use yew::{function_component, html, Html, Properties};
use yew_router::prelude::*;

use crate::app::MainRoute;

#[derive(PartialEq, Properties)]
pub struct EmphasisProps {
    pub emphasis: Value,
}

#[function_component]
pub fn Emphasis(props: &EmphasisProps) -> Html {
    let title = props.emphasis.get("subject").unwrap().as_str().unwrap();
    let summary = props.emphasis.get("summary").unwrap().as_str().unwrap();
    let slug = props
        .emphasis
        .get("slug")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    html! {
        <div class="p-4 p-md-5 mb-4 rounded text-body-emphasis bg-body-secondary">
          <div class="col-lg-6 px-0">
            <h1 class="display-4 fst-italic">{title}</h1>
            <p class="lead my-3">{summary}</p>
            <p class="lead mb-0">
            // <a href="#" class="text-body-emphasis fw-bold">{"Continue reading..."}</a>
              <Link<MainRoute> classes="text-body-emphasis fw-bold" to={MainRoute::ArticlePage {slug: slug.clone() }}>
                {"Continue reading..."}
              </Link<MainRoute>>
            </p>
          </div>
        </div>
    }
}

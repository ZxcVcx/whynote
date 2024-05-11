use crate::{app::ManageRoute, components::manage_container::content_card::ContentCard};
use serde_json::Value;
use yew::{function_component, html, Html, Properties};
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ContentsListProps {
    pub crafts: Vec<Value>,
    pub articles: Vec<Value>,
    // pub navigator: Navigator<MainRoute>,
}

#[function_component]
pub fn ContentsList(props: &ContentsListProps) -> Html {
    let ContentsListProps { crafts, articles } = props;
    html! {
    <div class="p-2">
        <div class="row">
            <div class="col-4"></div>
            <div class="col-4"></div>
            <div class="col-4">
                <Link<ManageRoute> to={ManageRoute::NewEditor }>
                    <button class="btn btn-primary">{"Create"}</button>
                </Link<ManageRoute>>
            </div>
        </div>
        <h3>{"crafts"}</h3>
        <ContentCard contents={crafts.clone()} published={false} />
        <h3>{"articles"}</h3>
        <ContentCard contents={articles.clone()} published={true} />
    </div>
    }
}

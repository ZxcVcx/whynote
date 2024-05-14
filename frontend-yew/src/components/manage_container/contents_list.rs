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
            <div class="col"></div>
            // <div class="col-4"></div>
            <div class="col d-flex justify-content-end align-items-center">
                <Link<ManageRoute> to={ManageRoute::NewEditor }>
                    <button class="btn btn-primary ms-2">{"Create"}</button>
                </Link<ManageRoute>>
                <Link<ManageRoute> to={ManageRoute::NewEditor }>
                    <button class="btn btn-secondary ms-2">{"Export"}</button>
                </Link<ManageRoute>>
                <Link<ManageRoute> to={ManageRoute::NewEditor }>
                    <button class="btn btn-danger ms-2">{"Delete All"}</button>
                </Link<ManageRoute>>
            </div>
        </div>
        <h3>{"crafts"}</h3>
        <ContentCard contents={crafts.clone()} published={false} />
        <h3 class="mt-5">{"articles"}</h3>
        <ContentCard contents={articles.clone()} published={true} />
    </div>
    }
}

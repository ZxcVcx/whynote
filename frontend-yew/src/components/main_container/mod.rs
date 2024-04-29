pub mod emphasis;
pub mod secondary;
pub mod article;
pub mod articles;
// pub mod main_container;

use serde_json::Value;
use yew::prelude::*;
// use emphasis::Emphasis;
// use secondary::Secondary;
use articles::Articles;
use emphasis::Emphasis;
use secondary::Secondary;

#[derive(PartialEq, Properties)]
pub struct MainContainerProps {
    pub emphasis: Value,
    pub secondary: Vec<Value>,
    pub recommanded: Vec<Value>,
}

#[function_component]
pub fn MainContainer(props: &MainContainerProps) -> Html {
    html! {
        <main class="container">
            <Emphasis emphasis={props.emphasis.clone()}/>
            <Secondary secondary={props.secondary.clone()}/>
            <Articles />
        </main>
    }
}

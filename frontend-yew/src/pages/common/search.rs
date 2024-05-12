use yew::{function_component, html, Html, Properties};
use crate::components::top_container::TopContainer;

#[derive(PartialEq, Properties)]
pub struct SearchPageProps {}

#[function_component]
pub fn SearchPage(props: &SearchPageProps) -> Html {
    let SearchPageProps {} = props;
    html! {
        <>
        <TopContainer />

        </>
    }
}
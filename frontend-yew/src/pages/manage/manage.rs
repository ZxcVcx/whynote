use yew::{function_component, html, Html, Properties};

use crate::components::top_container::manage_top_container::ManageTopContainer;

#[derive(PartialEq, Properties)]
pub struct ManagePageProps {}

#[function_component]
pub fn ManagePage(props: &ManagePageProps) -> Html {
    let ManagePageProps {} = props;
    html! {
        <ManageTopContainer />
    }
}

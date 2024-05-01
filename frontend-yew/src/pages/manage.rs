use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct ManagePageProps {}

#[function_component]
pub fn ManagePage(props: &ManagePageProps) -> Html {
    let ManagePageProps {} = props;
    html! {
        <div></div>
    }
}
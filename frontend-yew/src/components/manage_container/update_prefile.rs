use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct UpdateProfileProps {}

#[function_component]
pub fn UpdateProfile(props: &UpdateProfileProps) -> Html {
    let UpdateProfileProps {} = props;
    html! {
        <div></div>
    }
}
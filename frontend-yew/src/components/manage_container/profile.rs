use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct ProfileProps {
    
}

#[function_component]
pub fn Profile(props: &ProfileProps) -> Html {
    let ProfileProps {} = props;
    html! {
        <div></div>
    }
}
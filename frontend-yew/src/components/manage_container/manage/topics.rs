use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct TopicsManageComponentProps {}

#[function_component]
pub fn TopicsManageComponent(props: &TopicsManageComponentProps) -> Html {
    let TopicsManageComponentProps {} = props;
    html! {
        <div></div>
    }
}

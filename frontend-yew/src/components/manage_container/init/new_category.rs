use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct NewCategoryComponentProps {}

#[function_component]
pub fn NewCategoryComponent(props: &NewCategoryComponentProps) -> Html {
    let NewCategoryComponentProps {} = props;
    html! {
        <div></div>
    }
}
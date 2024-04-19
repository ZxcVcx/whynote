use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct UsersProps {}

#[function_component]
pub fn Users(props: &UsersProps) -> Html {
    let UsersProps {} = props;
    html! {
        <div>
        { "Users List - Blue" }
        </div>
    }
}
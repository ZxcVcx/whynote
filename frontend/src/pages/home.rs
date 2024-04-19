use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    let HomeProps {} = props;
    let home_cls = "home";

    html! {
        <div class={classes!(home_cls)}>
            { "Home page - Blue" }
        </div>
    }
}
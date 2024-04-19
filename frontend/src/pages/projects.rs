use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ProjectsProps {}

#[function_component]
pub fn Projects(props: &ProjectsProps) -> Html {
    let ProjectsProps {} = props;
    html! {
        <div class={classes!("projects")}>
            { "Projects List - Blue" }
        </div>
    }
}
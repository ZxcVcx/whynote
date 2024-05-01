use serde_json::Value;
use yew::prelude::*;
use crate::components::top_container::TopContainer;


use crate::components::main_container::emphasis::Emphasis;
use crate::components::main_container::secondary::Secondary;
use crate::components::main_container::main_container::MainContainer;

#[derive(PartialEq, Properties)]
pub struct HomePageProps {
    pub emphasis: Value,
    pub secondary: Vec<Value>,
    pub recommanded: Vec<Value>,
    pub about: Value,
    pub recent: Vec<Value>,
    // pub archives: Vec<Value>,
    // pub elsewhere: Vec<Value>,
}

#[function_component]
pub fn HomePage(props: &HomePageProps) -> Html {
    html! {
        <>
        <TopContainer />
        <main class="container">
            <Emphasis emphasis={props.emphasis.clone()}/>
            <Secondary secondary={props.secondary.clone()}/>
            // <Articles articles={props.recommanded.clone()}/>
            <MainContainer 
                recommanded={props.recommanded.clone()}
                about={props.about.clone()}
                recent={props.recent.clone()}
                // archives={props.archives.clone()}
                // elsewhere={props.elsewhere.clone()}
            />
        </main>
        </>
    }
}

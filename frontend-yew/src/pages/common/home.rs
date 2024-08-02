use crate::components::top_container::TopContainer;
use serde_json::Value;
use yew::prelude::*;

use crate::components::main_container::emphasis::Emphasis;
use crate::components::main_container::main_container::MainContainer;
use crate::components::main_container::secondary::Secondary;

#[derive(PartialEq, Properties)]
pub struct HomePageProps {
    // pub emphasis: Value,
    // pub secondary: Vec<Value>,
    pub top: Vec<Value>,
    pub recommended: Vec<Value>,
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
            {
                if props.top.is_empty() {
                    html! {
                        <MainContainer
                            recommended={props.recommended.clone()}
                            about={props.about.clone()}
                            recent={props.recent.clone()}
                            // archives={props.archives.clone()}
                            // elsewhere={props.elsewhere.clone()}
                        />
                    }
                } else if props.top.len() < 3 {
                    html! {
                        <>
                            <Emphasis emphasis={props.top[0].clone()}/>
                            <MainContainer
                                recommended={props.recommended.clone()}
                                about={props.about.clone()}
                                recent={props.recent.clone()}
                                // archives={props.archives.clone()}
                                // elsewhere={props.elsewhere.clone()}
                            />
                        </>
                    }
                } else {
                    html! {
                        <>
                            <Emphasis emphasis={props.top[0].clone()}/>
                            <Secondary secondary={props.top[1..].to_vec()}/>

                            <MainContainer
                                recommended={props.recommended.clone()}
                                about={props.about.clone()}
                                recent={props.recent.clone()}
                                // archives={props.archives.clone()}
                                // elsewhere={props.elsewhere.clone()}
                            />
                        </>
                    }
                }
            }
            // // <Emphasis emphasis={props.emphasis.clone()}/>
            // // <Secondary secondary={props.secondary.clone()}/>
            // // <Articles articles={props.recommended.clone()}/>
            // <MainContainer
            //     recommended={props.recommended.clone()}
            //     about={props.about.clone()}
            //     recent={props.recent.clone()}
            //     // archives={props.archives.clone()}
            //     // elsewhere={props.elsewhere.clone()}
            // />
        </main>
        </>
    }
}

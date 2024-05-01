use serde_json::Value;
use yew::prelude::*;

use super::articles::Articles;
use super::sticky::Sticky;

#[derive(PartialEq, Properties)]
pub struct MainContainerProps {
    pub recommanded: Vec<Value>,
    pub about: Value,
    pub recent: Vec<Value>,
    // pub archives: Vec<Value>,
    // pub elsewhere: Vec<Value>,
}

#[function_component]
pub fn MainContainer(props: &MainContainerProps) -> Html {
    
    html! {
        <div class="row g-5">
            <Articles articles={props.recommanded.clone()}/>
            // nav
            <Sticky 
                about={props.about.clone()} 
                recent={props.recent.clone()} 
                // archives={props.archives.clone()} 
                // elsewhere={props.elsewhere.clone()}
            />
        </div>
    }
}


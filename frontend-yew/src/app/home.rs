use crate::{
    app::ManageRoute,
    pages::common::home::HomePage,
    services::{articles::fetch_home_data, use_query::FetchError},
};
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::hooks::use_navigator;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let home_state = use_async(async { fetch_home_data().await });

    let effect_home_state = home_state.clone();
    let navigator = use_navigator().unwrap();
    // let emphasis = use_state(|| vec![]);
    // let secondary = use_state(|| vec![]);
    // let recommand = use_state(|| vec![]);

    use_effect_with((), move |_| {
        effect_home_state.run();
        || ()
    });

    html! {
        <>
            {
                if let Some(error) = &home_state.error {
                    if error == &FetchError::from(JsValue::from_str("Default user not found")) {
                        // TODO: Error collection
                        navigator.push(&ManageRoute::Init);
                        html! {
                            <div class="alert alert-danger" role="alert">
                                { "Default user not found, May be the database is empty" }
                            </div>
                        }
                    } else {
                        html! { error }
                    }
                } else {
                    html! {}

                }
            }
            {
                if let Some(home_data) = &home_state.data {
                    let top = home_data.get("topArticles").unwrap().as_array().unwrap().clone();                        // let emphasis = top_articles[0].clone();
                    // let secondary = vec![top_articles[1].clone(), top_articles[2].clone()];
                    let recommended = home_data.get("recommendedArticles").unwrap().as_array().unwrap().clone();
                    let about = home_data.get("about").unwrap().clone();
                    let recent = home_data.get("recentArticles").unwrap().as_array().unwrap().clone();

                    html! {
                        <HomePage
                            top={top}
                            // emphasis={emphasis}
                            // secondary={secondary}
                            recommended={recommended}
                            about={about}
                            recent={recent}
                        />
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}

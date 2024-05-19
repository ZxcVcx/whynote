use crate::app::MainRoute;
use crate::components::manage_container::init::new_articles::NewArticlesComponent;
use crate::components::manage_container::init::new_user::NewUserComponent;
use crate::services::articles::fetch_home_data;
use crate::types::init::UserNewToken;
use crate::types::init::{ArticleType, CategoryNewType};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
use yew_router::hooks::use_navigator;

#[derive(PartialEq, Properties)]
pub struct InitPageProps {}

#[function_component]
pub fn InitPage(props: &InitPageProps) -> Html {
    let InitPageProps {} = props;
    // let user_state = use_state(|| NewUser::empty());
    // let sign_info_state = use_state(|| SignInfo::empty());
    let navigator = use_navigator().unwrap().clone();

    let navigator_clone = navigator.clone();

    use_effect_with((), move |_| {
        let navigator = navigator_clone.clone();
        spawn_local(async move {
            let navigator = navigator.clone();
            let data = fetch_home_data().await;
            match data {
                Ok(_) => {
                    navigator.clone().push(&MainRoute::Home);
                }
                Err(_) => {}
            }
        });
        || ()
    });

    let user_new_token_state = use_state(|| UserNewToken::empty());
    let cetegories_data_state: yew::prelude::UseStateHandle<Vec<CategoryNewType>> =
        use_state(|| vec![]);
    let articles_data_state: yew::prelude::UseStateHandle<Vec<ArticleType>> = use_state(|| vec![]);
    let token_setter = user_new_token_state.setter();
    let articles_setter = articles_data_state.setter();
    let categories_setter = cetegories_data_state.setter();

    html! {
        <>
            {
                if !user_new_token_state.is_logged_in() {
                    html! { <NewUserComponent {token_setter} /> }
                } else if articles_data_state.is_empty() {
                    let user_new_token = (*user_new_token_state).clone();
                    html! {
                        <NewArticlesComponent {user_new_token} {articles_setter} {categories_setter} />
                    }
                } else {
                    let navigator = navigator.clone();
                    navigator.push(&MainRoute::Home);
                    html! {
                        <div>{"Loading..."}</div>
                    }
                }
            }
        </>
    }
}

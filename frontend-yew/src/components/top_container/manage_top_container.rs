use crate::app::{MainRoute, ManageRoute};
use crate::services::user::fetch_default_user_data;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use super::user_drop_down::UserDropDown;
use crate::utils::common::is_logged_in;

#[derive(PartialEq, Properties)]
pub struct HeaderProps {
    pub login_state: UseStateHandle<bool>,
}

#[function_component]
pub fn ManageHeader(props: &HeaderProps) -> Html {
    let login_state = props.login_state.clone();
    // let title = utils::constants::CFG.get("SITE_TITLE").unwrap().to_string();
    // let title = use_async(async move { fetch_default_user_data().await });
    let title = use_state_eq(|| "Blog Title".to_string());

    let effect_title = title.clone();
    let effect_login_state = login_state.clone();
    use_effect_with(effect_login_state.clone(), move |_| {
        spawn_local(async move {
            // let data = fetch_default_user_data().await;
            let title = match fetch_default_user_data().await {
                Ok(user) => user["defaultUser"]["blogName"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                Err(_) => "Blog Title".to_string(),
            };

            effect_title.set(title.clone());
        });
        || ()
    });

    let navigator = use_navigator().expect("Navigator should be available");

    let on_sign_in_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&MainRoute::SignIn))
    };

    html! {
    <header class="border-bottom lh-1 py-3">
      <div class="row flex-nowrap justify-content-between align-items-center">
        <div class="col-4">
          <Link<MainRoute> to={MainRoute::Home} classes="blog-header-logo text-body-emphasis text-decoration-none">
            <p>{title.to_string()}</p>
          </Link<MainRoute>>
        </div>
        <div class="col-4 d-flex justify-content-end align-items-center">
          // <a class="link-secondary" href="#" aria-label="Search">
          <Link<MainRoute> to={MainRoute::SearchPage} classes="text-body-emphasis">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" class="mx-3" role="img" viewBox="0 0 24 24"><title>{"Search"}</title><circle cx="10.5" cy="10.5" r="7.5"/><path d="M21 21l-5.2-5.2"/></svg>
          </Link<MainRoute>>

        {
            if *login_state {
              html! {
                <UserDropDown {login_state} />
              }
            } else {
               html! {
                   <div class="btn-group me-2">
                       <button type="button" onclick={on_sign_in_click} class="btn btn-sm btn-outline-secondary">{"Sign In"}</button>
                       // <button type="button" onclick={on_init_click}class="btn btn-sm btn-outline-secondary">{"Sign Up"}</button>
                   </div>
               }
            }
        }
        </div>
      </div>
    </header>
    }
}

/// Nav component
#[function_component]
pub fn ManageNav() -> Html {
    // let nav_list = use_async(async move { fetch_categories_list().await });

    // let effect_nav_list = nav_list.clone();

    // use_effect_with((), move |_| {
    //     effect_nav_list.run();
    //     || ()
    // });

    let nav_list = [
        // ("Manage", ManageRoute::Manage),
        ("Profile", ManageRoute::Profile),
        ("Categories", ManageRoute::Categories),
        ("Topics", ManageRoute::Topics),
        // ("Content", ManageRoute::Content),
    ];

    html! {
        <div class="nav-scroller py-1 mb-3 border-bottom">
            <nav class="nav nav-underline justify-content-between">
                <Link<MainRoute> classes="nav-item nav-link link-body-emphasis" to={MainRoute::Home}>
                    { "Home" }
                </Link<MainRoute>>
            {
                nav_list.iter().map(|(name, route)| {
                    web_sys::console::log_1(&JsValue::from_str(name));
                    html! {
                        <Link<ManageRoute> classes="nav-item nav-link link-body-emphasis" to={route.clone()}>
                            { name }
                        </Link<ManageRoute>>
                    }
                }).collect::<Html>()
            }
            </nav>

        </div>
    }
}

/// Nav component
#[function_component]
pub fn ManageTopContainer() -> Html {
    let login_state = use_state(is_logged_in);
    html! {
        <div class="container py-4">
            <ManageHeader {login_state}/>
            <ManageNav />
        </div>
    }
}

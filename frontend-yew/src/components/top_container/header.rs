use crate::{
    app::{MainRoute, ManageRoute},
    services::user::fetch_default_user_data,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use super::user_drop_down::UserDropDown;

#[derive(PartialEq, Properties)]
pub struct HeaderProps {
    pub login_state: UseStateHandle<bool>,
}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    let login_state = props.login_state.clone();
    // let title = utils::constants::CFG.get("SITE_TITLE").unwrap().to_string();
    // let title = use_async(async move { fetch_default_user_data().await });
    let title = use_state_eq(|| "Blog Title".to_string());

    let effect_title = title.clone();
    let effect_login_state = login_state.clone();
    use_effect_with(effect_login_state.clone(), move |_| {
        spawn_local(async move {
            // let data = fetch_default_user_data().await;
            let title = match  fetch_default_user_data().await {
                Ok(user) => user["defaultUser"]["blogName"].as_str().unwrap().to_string(),
                Err(_) => "Blog Title".to_string(),
            };

            effect_title.set(title.clone());
            // let data = fetch_categories_list()
            //     .await
            //     .unwrap()
            //     .get("categories")
            //     .unwrap()
            //     .as_array()
            //     .unwrap()
            //     .to_vec();
            // categories.set(data.clone());
        });
        || ()
        // async move {
        //     let data = fetch_default_user_data().await;
        //     match data {
        //         Ok(_) => {
        //             login_state.set(true);
        //         }
        //         Err(_) => {
        //             login_state.set(false);
        //         }
        //     }
        //     title.set("Blog Title".to_string());
        // }
    });

    let navigator = use_navigator().expect("Navigator should be available");

    let on_sign_in_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&ManageRoute::SignIn))
    };

    // let on_init_click = {
    //     let navigator = navigator.clone();
    //     Callback::from(move |_| navigator.push(&ManageRoute::Init))
    // };
    // let title = "Blog Title".to_string();
    html! {
    <header class="border-bottom lh-1 py-3">
      <div class="row flex-nowrap justify-content-between align-items-center">
        // <div class="col-4 pt-1">
        //   <a class="link-secondary" href="#">{"Subscribe"}</a>
        // </div>
        <div class="col-4 text-center">
          <Link<MainRoute> to={MainRoute::Home} classes="blog-header-logo text-body-emphasis text-decoration-none">
            // <p>{"test"}</p>
            <p>{title.to_string()}</p>
          </Link<MainRoute>>
        </div>
        <div class="col-4 d-flex justify-content-end align-items-center">
          // <a class="link-secondary" href="#" aria-label="Search">
          <Link<MainRoute> to={MainRoute::SearchPage} classes="text-body-emphasis">
            // <img src={create_gravatar_url("")}
            // {"Search"}
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" class="mx-3" role="img" viewBox="0 0 24 24"><title>{"Search"}</title><circle cx="10.5" cy="10.5" r="7.5"/><path d="M21 21l-5.2-5.2"/></svg>
          // </a>
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

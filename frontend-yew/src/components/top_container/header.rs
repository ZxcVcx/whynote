use crate::app::ManageRoute;
use crate::utils;
use yew::prelude::*;
use yew_router::prelude::*;

use super::user_drop_down::UserDropDown;

#[derive(PartialEq, Properties)]
pub struct HeaderProps {
    pub login_state: UseStateHandle<bool>
    // pub archives: Vec<Value>,
    // pub elsewhere: Vec<Value>,
}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    let login_state = props.login_state.clone();
    let title = utils::constants::CFG.get("SITE_TITLE").unwrap().to_string();
    let navigator = use_navigator().expect("Navigator should be available");
    

    let on_sign_in_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&ManageRoute::SignIn))
    };

    let on_sign_up_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&ManageRoute::SignUp))
    };
    // let title = "Blog Title".to_string();
    html! {
    <header class="border-bottom lh-1 py-3">
      <div class="row flex-nowrap justify-content-between align-items-center">
        // <div class="col-4 pt-1">
        //   <a class="link-secondary" href="#">{"Subscribe"}</a>
        // </div>
        <div class="col-4 text-center">
          <a class="blog-header-logo text-body-emphasis text-decoration-none" href="/">{title}</a>
        </div>
        <div class="col-4 d-flex justify-content-end align-items-center">
          <a class="link-secondary" href="#" aria-label="Search">
            // <img src={create_gravatar_url("")}
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" class="mx-3" role="img" viewBox="0 0 24 24"><title>{"Search"}</title><circle cx="10.5" cy="10.5" r="7.5"/><path d="M21 21l-5.2-5.2"/></svg>
          </a>

        {
            if *login_state {
              html! {
                <UserDropDown {login_state} />
                // <a class="link-secondary" href="#" aria-label="Profile">
                // //   <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" class="mx-3" role="img" viewBox="0 0 24 24"><title>{"Profile"}</title><circle cx="12" cy="12" r="10"/><path d="M16 12l-4 4-4-4"/></svg>
                // <img class="rounded-circle" src={create_gravatar_url(get_pair_value("email").unwrap().as_str(), 80)} width="40" />
                // </a>
              }
            } else {
                html! {
                    <div class="btn-group me-2">
                        <button type="button" onclick={on_sign_in_click} class="btn btn-sm btn-outline-secondary">{"Sign In"}</button>
                        <button type="button" onclick={on_sign_up_click}class="btn btn-sm btn-outline-secondary">{"Sign Up"}</button>
                    </div>
                }
            }
        }

        //   <div class="btn-group me-2">
        //     <button type="button" onclick={on_sign_in_click} class="btn btn-sm btn-outline-secondary">{"Sign In"}</button>
        //     <button type="button" onclick={on_sign_up_click}class="btn btn-sm btn-outline-secondary">{"Sign Up"}</button>
        //   </div>
        </div>
      </div>
    </header>
    }
}

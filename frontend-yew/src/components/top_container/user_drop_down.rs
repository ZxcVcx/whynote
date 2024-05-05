use crate::app::MainRoute;
use crate::utils::storage::{self, get_pair_value};
use crate::{app::ManageRoute, utils::common::create_gravatar_url};
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct UserDropDownProps {
    pub login_state: UseStateHandle<bool>,
}

#[function_component]
pub fn UserDropDown(props: &UserDropDownProps) -> Html {
    let login_state = props.login_state.clone();
    // let logout = {
    //     let router = use_router();
    //     Callback::from(move |_| {
    //         router.send_message(crate::app::Msg::Logout);
    //     })
    // };
    let log_out = {
        let navigator = use_navigator().expect("Navigator should be available");

        Callback::from(move |_| {
            storage::remove_pair("jwt");
            storage::get_pair_value("email");
            login_state.set(false);
            navigator.push(&MainRoute::Home);
        })
    };
    html! {
        <div class="dropdown">
            // <button class="btn btn-secondary dropdown-toggle" type="button" data-bs-toggle="dropdown" aria-expanded="false">
            //     <img class="rounded-circle" src={create_gravatar_url(get_pair_value("email").unwrap().as_str(), 80)} width="40" />
            // </button>
            <img class="rounded-circle" role="button" data-bs-toggle="dropdown" aria-expanded="false" src={create_gravatar_url(get_pair_value("email").unwrap().as_str(), 80)} width="40" />
            <ul class="dropdown-menu dropdown-menu-end">
                <li>
                    <Link<ManageRoute> classes="dropdown-item" to={ManageRoute::Profile}>
                        { "Profile" }
                    </Link<ManageRoute>>
                </li>
                <li>
                    <Link<ManageRoute> classes="dropdown-item" to={ManageRoute::Content}>
                        { "Content" }
                    </Link<ManageRoute>>
                </li>
                <li>
                    // <a class="dropdown-item" href="#">{"Sign Out"}</a>
                    // <Link<ManageRoute> classes="dropdown-item" to={ManageRoute::Profile}>
                    //     { "Sign Out" }
                    // </Link<ManageRoute>>
                    <button class = "dropdown-item" onclick={log_out}>
                        { "Sign Out" }
                    </button>
                </li>
            </ul>
        </div>
        // <>
        //     <a class="link-secondary" href="#" aria-label="Profile">
        //         <img class="rounded-circle" src={create_gravatar_url(get_pair_value("email").unwrap().as_str(), 80)} width="40" />
        //     </a>
        // </>
    }
}

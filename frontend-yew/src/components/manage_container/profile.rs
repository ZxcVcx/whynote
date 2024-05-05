use serde_json::Value;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement, InputEvent};
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
// use yew_router::hooks::use_navigator;

// use crate::app::MainRoute;
use crate::services::user::user_update_profile_data;
// use crate::utils::common::{is_logged_in, log_out};
use crate::utils::{common::create_gravatar_url, storage::get_pair_value};

#[derive(PartialEq, Properties)]
pub struct ProfileProps {
    pub user: Value,
}

#[function_component]
pub fn Profile(props: &ProfileProps) -> Html {
    // let navigator = use_navigator().unwrap();
    // let ProfileProps {use} = props;
    let user = props.user.clone();
    let init_email = user["email"].as_str().unwrap();
    // let id = user["id"].clone();
    let init_username = user["username"].as_str().unwrap();
    let init_nickname = user["nickname"].as_str().unwrap();
    // let blog_name = user["blogName"].clone();
    let init_blog_name = user["blogName"].as_str().unwrap();
    let init_website = user["website"].as_str().unwrap();
    let init_bio = user["bio"].as_str().unwrap();

    let email = use_state(|| init_email.to_string());
    let username = use_state(|| init_username.to_string());
    let nickname = use_state(|| init_nickname.to_string());
    let blog_name = use_state(|| init_blog_name.to_string());
    let website = use_state(|| init_website.to_string());
    let bio = use_state(|| init_bio.to_string());

    let change_state_flag = use_state(|| false);

    let on_edit_click = {
        let change_state_flag = change_state_flag.clone();
        Callback::from(move |_| {
            change_state_flag.set(true);
        })
    };

    let on_username_change = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_nickname_change = {
        let nickname = nickname.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            nickname.set(input.value());
        })
    };

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_website_change = {
        let website = website.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            website.set(input.value());
        })
    };

    let on_blog_name_change = {
        let blog_name = blog_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            blog_name.set(input.value());
        })
    };

    let on_bio_change = {
        let bio = bio.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            bio.set(input.value());
        })
    };

    let on_save_click = {
        // let navigator = navigator.clone();
        let email = email.clone();
        let username = username.clone();
        let nickname = nickname.clone();
        let blog_name = blog_name.clone();
        let website = website.clone();
        let bio = bio.clone();
        let change_state_flag = change_state_flag.clone();

        Callback::from(move |_| {
            // let navigator = navigator.clone();
            let email = email.clone();
            let username = username.clone();
            let nickname = nickname.clone();
            let blog_name = blog_name.clone();
            let website = website.clone();
            let bio = bio.clone();

            spawn_local(async move {
                // let navigator = navigator.clone();
                let token = get_pair_value("jwt").unwrap();
                let _data = user_update_profile_data(
                    (*email).clone(),
                    (*username).clone(),
                    (*nickname).clone(),
                    (*blog_name).clone(),
                    (*website).clone(),
                    (*bio).clone(),
                    token,
                )
                .await
                .unwrap();
            });

            change_state_flag.set(false);
            web_sys::window().unwrap().location().reload().unwrap();
            // web_sys::console::log_1(&JsValue::from_str(&(*email).clone()));
        })
    };

    let on_cancel_click = {
        let email = email.clone();
        let username = username.clone();
        let nickname = nickname.clone();
        let blog_name = blog_name.clone();
        let website = website.clone();
        let bio = bio.clone();
        let change_state_flag = change_state_flag.clone();
        Callback::from(move |_| {
            web_sys::console::log_3(
                &JsValue::from_str(&(*email).clone()),
                &JsValue::from_str(user["email"].as_str().unwrap()),
                &JsValue::from_bool((*email).clone() == user["email"].as_str().unwrap()),
            );

            email.set(user["email"].as_str().unwrap().to_string());
            username.set(user["username"].as_str().unwrap().to_string());
            nickname.set(user["nickname"].as_str().unwrap().to_string());
            blog_name.set(user["blogName"].as_str().unwrap().to_string());
            website.set(user["website"].as_str().unwrap().to_string());
            bio.set(user["bio"].as_str().unwrap().to_string());

            change_state_flag.set(false);
            // web_sys::console::log_3(&JsValue::from_str(&(*email).clone()), &JsValue::from_str(init_email), &JsValue::from_bool((*email).clone() == init_email));
            // web_sys::console::log_1(&JsValue::from_str(&(*username).clone()));
        })
    };

    html! {
        <div class="col-md-3">
            <div>
                <div class="p-2">
                    <img class="profile-avatar w-100 img-fluid rounded-circle" src={create_gravatar_url(get_pair_value("email").unwrap_or("".to_string()).as_str(), 150)} width="150px"/>
                </div>
                <div >
                    {
                        if !*change_state_flag.clone() {
                            html! {
                                <>
                                <div class="p-2">
                                    <h4>{(*nickname).clone()}</h4>
                                    <p>{(*username).clone()}</p>
                                    <p>{(*bio).clone()}</p>
                                    <button onclick={on_edit_click} class="w-100 btn btn-secondary">{"Edit Profile"}</button>
                                </div>
                                <div class="p-2">
                                    <div>{"Started 2024"}</div>
                                    <div>{(*email).clone()}</div>
                                    <a href={(*website).clone()}>{(*website).clone()}</a>
                                </div>
                                </>
                            }
                        } else {
                            html! {
                                <>
                                    <div class="input-group mb-3">
                                      <span class="input-group-text" id="basic-addon1">{"@"}</span>
                                      <input type="text" class="form-control" placeholder="Username" aria-label="Username" aria-describedby="basic-addon1"
                                        disabled=true
                                        oninput={on_username_change}
                                        value={{(*username).clone()}}
                                      />
                                    </div>

                                    <div class="input-group mb-3">
                                      <input type="text" class="form-control" placeholder="Nickname" aria-label="Nickname" aria-describedby="basic-addon1"
                                        oninput={on_nickname_change}
                                        value={{(*nickname).clone()}}
                                      />
                                    </div>

                                    <div class="input-group mb-3">
                                      <input type="email" class="form-control" placeholder="name@example.com" aria-label="Email" aria-describedby="basic-addon2"
                                        oninput={on_email_change}
                                        value={{(*email).clone()}}
                                      />
                                    //   <span class="input-group-text" id="basic-addon2">{"@example.com"}</span>
                                    </div>

                                    <div class="input-group mb-3">
                                      <input type="text" class="form-control" placeholder="Blog" aria-label="BlogName" aria-describedby="basic-addon2"
                                        oninput={on_blog_name_change}
                                        value={{(*blog_name).clone()}}
                                      />
                                    //   <span class="input-group-text" id="basic-addon2">{"@example.com"}</span>
                                    </div>

                                    <div class="input-group">
                                      <span class="input-group-text">{"Bio"}</span>
                                      <textarea class="form-control" aria-label="Bio"
                                        oninput={on_bio_change}
                                        value={{(*bio).clone()}}
                                      ></textarea>
                                    </div>

                                    <div class="mb-3">
                                      <label for="basic-url" class="form-label">{"Your Website"}</label>
                                      <input type="uri" class="form-control" placeholder="Your Website" aria-label="Nickname" aria-describedby="basic-addon1"
                                        oninput={on_website_change}
                                        value={{(*website).clone()}}
                                      />
                                    </div>

                                    <div class="row">
                                        <div class="col d-grid gap-2">
                                      <button onclick={on_save_click} class="btn btn-primary" type="button">{"Save"}</button>
                                    </div>

                                    <div class="col d-grid gap-2">
                                      <button onclick={on_cancel_click} class="btn btn-secondary" type="button">{"Cancel"}</button>
                                    </div>
                                    </div>
                                </>
                            }
                        }
                    }

                </div>

            </div>
        </div>
    }
}

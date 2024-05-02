use crate::{app::MainRoute, services::signin::fetch_sign_in_data, utils::token::store_pair};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_state, Callback, Html, InputEvent, Properties, SubmitEvent,
    TargetCast,
};
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SignInProps {}

#[function_component]
pub fn SignIn(props: &SignInProps) -> Html {
    let SignInProps {} = props;
    let navigator = use_navigator().unwrap();
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let remember_me = use_state(|| false);

    let on_username_change = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let navigator = navigator.clone();
        let username = username.clone();
        let password = password.clone();
        let remember_me = remember_me.clone();
        // Callback::from(move |e: FocusEvent| {
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            // log::info!("Logging in with username: {} and password: {}", *username, *password);
            let navigator = navigator.clone();
            let username = username.clone();
            let password = password.clone();
            let remember_me = remember_me.clone();
            spawn_local(async move {
                let result = fetch_sign_in_data(username.to_string(), password.to_string()).await;
                match result {
                    Ok(data) => {
                        // web_sys::console::log_1(&format!("Data: {:?}", data).into());
                        let token = data["userSignIn"]["token"].as_str().unwrap();
                        store_pair("jwt", token, *remember_me);
                        let email = data["userSignIn"]["email"].as_str().unwrap();
                        store_pair("email", email, *remember_me);
                        navigator.push(&MainRoute::Home)
                    }
                    Err(e) => {
                        // log::error!("Error: {:?}", e);
                        web_sys::console::log_1(&format!("Error: {:?}", e).into());
                    }
                }
            });
        })
    };

    html! {
        <>
        <main class="form-signin w-100 m-auto">
            <form onsubmit={on_submit}>
                // <img class="mb-4" src="../assets/brand/bootstrap-logo.svg" alt="" width="72" height="57">
                 <h1 class="h3 mb-3 fw-normal">{"Please sign in"}</h1>

                <div class="form-floating">
                    <input type="email" class="form-control" id="floatingInput" placeholder="name@example.com"
                        value={(*username).clone()}
                        oninput={on_username_change}
                    />
                    <label for="floatingInput">{"Email address"}</label>
                </div>
                <div class="form-floating">
                    <input type="password" class="form-control" id="floatingPassword" placeholder="Password"
                        value={(*password).clone()}
                        oninput={on_password_change}
                    />
                    <label for="floatingPassword">{"Password"}</label>
                </div>

                <div class="form-check text-start my-3">
                    <input class="form-check-input" type="checkbox" value="remember-me" id="flexCheckDefault"
                        checked={*remember_me}
                        onchange={Callback::from(move |_| {
                            remember_me.set(!*remember_me);
                        })}
                    />
                    <label class="form-check-label" for="flexCheckDefault">
                        {"Remember me"}
                    </label>
                </div>
                <button class="btn btn-primary w-100 py-2" type="submit">{"Sign in"}</button>
                // <p class="mt-5 mb-3 text-body-secondary">&copy; 2017–2024</p>
            </form>
        </main>
        </>
    }
}

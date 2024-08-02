use crate::services::init::fetch_new_user_token_data_typed;
use crate::types::init::{NewUser, UserNewToken};
use crate::utils::storage::store_pair;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};

#[derive(Clone)]
struct DivShows {
    email: bool,
    password: bool,
    username: bool,
    nickname: bool,
    blog_name: bool,
    blog_description: bool,
    elsewhere: bool,
}
impl DivShows {
    fn all_hide() -> Self {
        Self {
            email: false,
            password: false,
            username: false,
            nickname: false,
            blog_name: false,
            blog_description: false,
            elsewhere: false,
        }
    }
}

#[derive(Clone)]
struct ContinueShows {
    email: bool,
    password: bool,
    username: bool,
    nickname: bool,
    blog_name: bool,
    blog_description: bool,
    elsewhere: bool,
    submit: bool,
}
impl ContinueShows {
    fn all_hide() -> Self {
        Self {
            email: false,
            password: false,
            username: false,
            nickname: false,
            blog_name: false,
            blog_description: false,
            elsewhere: false,
            submit: false,
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct NewUserComponentProps {
    // pub on_new_user_submit: Callback<UserNewToken>,
    // pub user_new_token: UseStateHandle<>
    pub token_setter: UseStateSetter<UserNewToken>,
}

#[function_component]
pub fn NewUserComponent(props: &NewUserComponentProps) -> Html {
    // let NewUserComponentProps {token_setter} = props;
    let token_setter = props.token_setter.clone();
    let user_state = use_state(NewUser::empty);
    let token_info_state = use_state(UserNewToken::empty);
    let remember_me = use_state(|| true);

    let input_hidden = classes!(vec!["input-group", "mb-1", "visually-hidden"]);

    let input_show = classes!(vec!["input-group", "mb-1"]);

    let continue_hidden = classes!(vec!["btn", "btn-outline-secondary", "visually-hidden"]);

    let continue_show = classes!(vec!["btn", "btn-outline-secondary"]);

    let inputs_show_state = use_state(|| DivShows {
        email: true,
        ..DivShows::all_hide()
    });

    let continue_show_state = use_state(ContinueShows::all_hide);

    let on_email_change = {
        let user = user_state.clone();
        let continue_show_state = continue_show_state.clone();
        Callback::from(move |e: InputEvent| {
            let user = user.clone();
            let continue_show_state = continue_show_state.clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            // user.set(input.value());
            user.set(NewUser {
                email: input.value(),
                ..(*user).clone()
            });
            // if the length of user's email > 0, show continue button
            if !input.value().is_empty() {
                continue_show_state.set(ContinueShows {
                    email: true,
                    ..ContinueShows::all_hide()
                })
            }
        })
    };

    let on_email_continue = {
        let inputs_show_state = inputs_show_state.clone();
        let continue_show_state = continue_show_state.clone();
        Callback::from(move |_: MouseEvent| {
            let inputs_show_state = inputs_show_state.clone();
            let continue_show_state = continue_show_state.clone();
            inputs_show_state.set(DivShows {
                password: true,
                ..(*inputs_show_state).clone()
            });
            continue_show_state.set(ContinueShows {
                password: true,
                ..ContinueShows::all_hide()
            });
        })
    };

    let on_password_change = {
        let user = user_state.clone();
        let continue_show_state = continue_show_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let user = user.clone();
            let continue_show_state = continue_show_state.clone();
            // user.set(input.value());
            user.set(NewUser {
                password: input.value(),
                ..(*user).clone()
            });
            if !input.value().is_empty() {
                continue_show_state.set(ContinueShows {
                    password: true,
                    ..ContinueShows::all_hide()
                })
            }
        })
    };

    let on_password_continue = {
        let inputs_show_state = inputs_show_state.clone();
        let continue_show_state = continue_show_state.clone();
        Callback::from(move |_: MouseEvent| {
            let inputs_show_state = inputs_show_state.clone();
            let continue_show_state = continue_show_state.clone();
            inputs_show_state.set(DivShows {
                username: true,
                ..(*inputs_show_state).clone()
            });
            continue_show_state.set(ContinueShows {
                username: true,
                ..ContinueShows::all_hide()
            });
        })
    };

    let on_username_change = {
        let user = user_state.clone();
        let continue_show_state = continue_show_state.clone();
        Callback::from(move |e: InputEvent| {
            let user = user.clone();
            let continue_show_state = continue_show_state.clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            // user.set(input.value());
            user.set(NewUser {
                username: input.value(),
                ..(*user).clone()
            });
            if !input.value().is_empty() {
                continue_show_state.set(ContinueShows {
                    username: true,
                    ..ContinueShows::all_hide()
                })
            }
        })
    };

    let on_username_continue = {
        let inputs_show_state = inputs_show_state.clone();
        let continue_show_state = continue_show_state.clone();
        Callback::from(move |_: MouseEvent| {
            let inputs_show_state = inputs_show_state.clone();
            let continue_show_state = continue_show_state.clone();
            inputs_show_state.set(DivShows {
                nickname: true,
                ..(*inputs_show_state).clone()
            });
            continue_show_state.set(ContinueShows {
                nickname: true,
                ..ContinueShows::all_hide()
            });
        })
    };

    let on_nickname_change = {
        let user = user_state.clone();
        let continue_show_state = continue_show_state.clone();
        Callback::from(move |e: InputEvent| {
            let user = user.clone();
            let continue_show_state = continue_show_state.clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            // user.set(input.value());
            user.set(NewUser {
                nickname: input.value(),
                ..(*user).clone()
            });
            if !input.value().is_empty() {
                continue_show_state.set(ContinueShows {
                    nickname: true,
                    ..ContinueShows::all_hide()
                })
            }
        })
    };

    let on_nickname_continue = {
        let inputs_show_state = inputs_show_state.clone();
        let continue_show_state = continue_show_state.clone();
        Callback::from(move |_: MouseEvent| {
            let inputs_show_state = inputs_show_state.clone();
            let continue_show_state = continue_show_state.clone();
            inputs_show_state.set(DivShows {
                blog_name: true,
                ..(*inputs_show_state).clone()
            });
            continue_show_state.set(ContinueShows {
                blog_name: true,
                ..ContinueShows::all_hide()
            });
        })
    };

    let on_blog_name_change = {
        let user = user_state.clone();
        let continue_show_state = continue_show_state.clone();
        Callback::from(move |e: InputEvent| {
            let user = user.clone();
            let continue_show_state = continue_show_state.clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            // user.set(input.value());
            user.set(NewUser {
                blog_name: input.value(),
                ..(*user).clone()
            });
            if !input.value().is_empty() {
                continue_show_state.set(ContinueShows {
                    blog_name: true,
                    ..ContinueShows::all_hide()
                })
            }
        })
    };

    let on_blog_name_continue = {
        let inputs_show_state = inputs_show_state.clone();
        let continue_show_state = continue_show_state.clone();
        Callback::from(move |_: MouseEvent| {
            let inputs_show_state = inputs_show_state.clone();
            let continue_show_state = continue_show_state.clone();
            inputs_show_state.set(DivShows {
                blog_description: true,
                ..(*inputs_show_state).clone()
            });
            continue_show_state.set(ContinueShows {
                blog_description: true,
                ..ContinueShows::all_hide()
            });
        })
    };

    let on_blog_description_change = {
        let user = user_state.clone();
        let continue_show_state = continue_show_state.clone();
        Callback::from(move |e: InputEvent| {
            let user = user.clone();
            let continue_show_state = continue_show_state.clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            // user.set(input.value());
            user.set(NewUser {
                blog_description: input.value(),
                ..(*user).clone()
            });
            if !input.value().is_empty() {
                continue_show_state.set(ContinueShows {
                    blog_description: true,
                    ..ContinueShows::all_hide()
                })
            }
        })
    };

    let on_blog_description_continue = {
        let inputs_show_state = inputs_show_state.clone();
        let continue_show_state = continue_show_state.clone();
        Callback::from(move |_: MouseEvent| {
            let inputs_show_state = inputs_show_state.clone();
            let continue_show_state = continue_show_state.clone();
            inputs_show_state.set(DivShows {
                elsewhere: true,
                ..(*inputs_show_state).clone()
            });
            continue_show_state.set(ContinueShows {
                elsewhere: true,
                ..ContinueShows::all_hide()
            });
        })
    };

    let on_elsewhere_change = {
        let user = user_state.clone();
        let continue_show_state = continue_show_state.clone();
        Callback::from(move |e: InputEvent| {
            let user = user.clone();
            let continue_show_state = continue_show_state.clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            // user.set(input.value());
            user.set(NewUser {
                elsewhere: input.value(),
                ..(*user).clone()
            });
            if !input.value().is_empty() {
                continue_show_state.set(ContinueShows {
                    elsewhere: true,
                    ..ContinueShows::all_hide()
                })
            }
        })
    };

    let on_elsewhere_continue = {
        let continue_show_state = continue_show_state.clone();
        Callback::from(move |_: MouseEvent| {
            let continue_show_state = continue_show_state.clone();
            continue_show_state.set(ContinueShows {
                submit: true,
                ..ContinueShows::all_hide()
            });
        })
    };

    let on_submit = {
        let new_user = user_state.clone();
        let remember_me = remember_me.clone();
        // Callback::from(move |e: FocusEvent| {
        let token_info_state = token_info_state.clone();
        let token_setter = token_setter.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            // log::info!("Logging in with username: {} and password: {}", *username, *password);
            let new_user = new_user.clone();
            let token_info_state = token_info_state.clone();
            let remember_me = remember_me.clone();
            let token_setter = token_setter.clone();
            spawn_local(async move {
                // TODO
                let result = fetch_new_user_token_data_typed(&(*new_user).clone()).await;
                match result {
                    Ok(data) => {
                        // web_sys::console::log_1(&format!("Data: {:?}", data).into());
                        let id = data["userNewToken"]["id"].as_str().unwrap();
                        let email = data["userNewToken"]["email"].as_str().unwrap();
                        let username = data["userNewToken"]["username"].as_str().unwrap();
                        let token = data["userNewToken"]["token"].as_str().unwrap();
                        store_pair("jwt", token, *remember_me);
                        store_pair("email", email, *remember_me);
                        token_info_state.set(UserNewToken {
                            id: id.to_string(),
                            email: email.to_string(),
                            username: username.to_string(),
                            token: token.to_string(),
                        });
                        token_setter.set(UserNewToken {
                            id: id.to_string(),
                            email: email.to_string(),
                            username: username.to_string(),
                            token: token.to_string(),
                        });

                        // navigator.push(&MainRoute::Home)
                    }
                    Err(e) => {
                        // log::error!("Error: {:?}", e);
                        web_sys::console::log_1(&format!("Error: {:?}", e).into());
                        // error.set(e);
                    }
                }
            });
        })
    };

    html! {
        <div class="w-100 h-100 bg-body-tertiary">
        <main class="bg-body-tertiary form-signup h-100 w-100 m-auto d-flex align-items-center justify-content-center">
            <form class="w-100" onsubmit={on_submit}>
                // <img class="mb-4" src="../assets/brand/bootstrap-logo.svg" alt="" width="72" height="57">
                 <h1 class="h3 mb-3 fw-normal">{"Please Set Some Info First"}</h1>

                <div class={match inputs_show_state.email { true => {input_show.clone()}, false => {input_hidden.clone()} }}>
                    <div class="form-floating">
                        <input type="email" class="form-control" id="floatingEmail" placeholder="example@example.com"
                            value={user_state.email.clone()}
                            oninput={on_email_change}
                            // onfocus={on_email_focus}
                            required=true
                        />
                        <label for="floatingEmail">{"Email address"}</label>
                    </div>
                    <button class={ match continue_show_state.email {true => {continue_show.clone()}, false => {continue_hidden.clone()}}} onclick={on_email_continue} type="button">{"Continue"}</button>
                </div>


                <div class={match inputs_show_state.password { true => {input_show.clone()}, false => {input_hidden.clone()} }}>
                    <div class="form-floating">
                        <input type="password" class="form-control" id="floatingPassword" placeholder="Password"
                            value={user_state.password.clone()}
                            oninput={on_password_change}
                            required=true
                        />
                        <label for="floatingPassword">{"Password"}</label>
                    </div>
                    <button class={ match continue_show_state.password {true => {continue_show.clone()}, false => {continue_hidden.clone()}}} onclick={on_password_continue} type="button">{"Continue"}</button>
                </div>
                <div class={match inputs_show_state.username { true => {input_show.clone()}, false => {input_hidden.clone()} }}>
                    <div class="form-floating">
                        <input type="text" class="form-control" id="floatingUsername" placeholder="nickname"
                            value={user_state.username.clone()}
                            oninput={on_username_change}
                            required=true
                        />
                        <label for="floatingNickname">{"Username"}</label>
                    </div>
                    <button class={ match continue_show_state.username  {true => {continue_show.clone()}, false => {continue_hidden.clone()}}} onclick={on_username_continue} type="button">{"Continue"}</button>
                </div>

                <div class={match inputs_show_state.nickname { true => {input_show.clone()}, false => {input_hidden.clone()} }}>
                    <div class="form-floating">
                        <input type="text" class="form-control" id="floatingNickname" placeholder="nickname"
                            value={user_state.nickname.clone()}
                            oninput={on_nickname_change}
                            required=true
                        />
                        <label for="floatingNickname">{"Nickname"}</label>
                    </div>
                    <button class={match continue_show_state.nickname {true => {continue_show.clone()}, false => {continue_hidden.clone()}}} onclick={on_nickname_continue} type="button">{"Continue"}</button>
                </div>

                <div class={match inputs_show_state.blog_name { true => {input_show.clone()}, false => {input_hidden.clone()} }}>
                    <div class="form-floating">
                        <input type="text" class="form-control" id="floatingBlogName" placeholder="blog name"
                            value={user_state.blog_name.clone()}
                            oninput={on_blog_name_change}
                            required=true
                        />
                        <label for="floatingBlogName">{"Blog Name"}</label>
                    </div>
                    <button class={match continue_show_state.blog_name {true => {continue_show.clone()}, false => {continue_hidden.clone()}}} onclick={on_blog_name_continue} type="button">{"Continue"}</button>
                </div>

                <div class={match inputs_show_state.blog_description { true => {input_show.clone()}, false => {input_hidden.clone()} }}>
                    // <span class="input-group-text">{"Description"}</span>
                    <textarea rows="4" class="form-control" id="floatingBlogDescription" placeholder="blog description"
                        value={user_state.blog_description.clone()}
                        oninput={on_blog_description_change}
                        required=true
                    />
                    <button class={match continue_show_state.blog_description {true => {continue_show.clone()}, false => {continue_hidden.clone()}}} onclick={on_blog_description_continue} type="button">{"Continue"}</button>
                    // <label for="floatingBlogDescription">{"Blog Description"}</label>
                </div>

                <div class={match inputs_show_state.elsewhere { true => {input_show.clone()}, false => {input_hidden.clone()} }}>
                    <div class="form-floating">
                        <input type="text" class="form-control" id="floatingElsewhere" placeholder="elsewhere"
                            value={user_state.elsewhere.clone()}
                            oninput={on_elsewhere_change}
                        />
                        <label for="floatingElsewhere">{"Elsewhere (Optional)"}</label>
                    </div>
                    <button class={match continue_show_state.elsewhere {true => {continue_show.clone()}, false => {continue_hidden.clone()}}} onclick={on_elsewhere_continue} type="button">{"Continue"}</button>
                    // <label for="floatingBlogDescription">{"Blog Description"}</label>
                </div>

                // <button class="btn btn-primary w-100 py-2" type="submit">{"Next Step"}</button>
                <button class={match continue_show_state.submit {true => {classes!("w-100", "py-2", continue_show.clone())}, false => {continue_hidden.clone()}}} type="submit">{"Next Step"}</button>
                // <p class="mt-5 mb-3 text-body-secondary">&copy; 2017–2024</p>
            </form>
        </main>
        </div>
    }
}

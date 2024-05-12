use yew::{function_component, html, Html, Properties};

use crate::app::MainRoute;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;
// use crate::services::use_query::FetchError;
use crate::services::user::fetch_user_data_by_email;

use crate::components::manage_container::contents_list::ContentsList;
use crate::components::manage_container::profile::Profile;
use crate::utils::common::log_out;
use crate::utils::storage::get_pair_value;

#[derive(PartialEq, Properties)]
pub struct ProfilePageProps {}

#[function_component]
pub fn ProfilePage(props: &ProfilePageProps) -> Html {
    let ProfilePageProps {} = props;
    let navigator = use_navigator().unwrap();
    // if get_pair_value("jwt").is_none() {
    //     // web_sys::window().unwrap().alert_with_message("Please login first!");
    //     web_sys::window().unwrap().location().set_href("/").unwrap();
    //     // return html! {};
    // };
    // let email = use_state(|| get_pair_value("email"));
    let email_state = use_state(|| get_pair_value("email"));
    let user_state = use_async(async {
        // TODO: email logic
        let email = match get_pair_value("email") {
            Some(email) => email,
            None => "".to_string(),
        };
        fetch_user_data_by_email(email).await
    });

    let effect_user_state = user_state.clone();
    use_effect_with(email_state, move |_| {
        effect_user_state.run();
        || ()
    });
    // use_effect_with()
    html! {
        <>
            {
                if user_state.loading {
                    html! { "Loading" }
                } else {
                    html! {
                    }
                }
            }
            {
                if let Some(error) = &user_state.error {
                    log_out();
                    let navigator = navigator.clone();
                    navigator.push(&MainRoute::Home);
                    html! {error}
                } else {
                    html! {}
                }
            }
            {
                if let Some(user_data) = &user_state.data {
                    let user = user_data["userByEmail"].clone();
                    // let banned = user["banned"].clone();
                    let crafts = user["crafts"].as_array().unwrap();
                    let articles = user["articles"].as_array().unwrap();
                    // let carft = user_data["carft"].clone().as_array().unwrap();
                    // let published = user_data["published"].clone().as_array().unwrap();
                    html! {
                        <main class="container" style="min-height:100%;">
                            <div class="row g-1">
                                <Profile user={user.clone()} />
                                <div class="col-md-9">
                                    <ContentsList crafts={crafts.clone()} articles={articles.clone()} />
                                </div>
                            </div>
                        </main>
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}

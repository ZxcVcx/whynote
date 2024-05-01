use yew::{function_component, html, Html, Properties};
use yew::prelude::*;
use yew_hooks::prelude::*;
use crate::services::user::fetch_user_data_by_username;
use crate::components::main_container::articles_list::ArticlesList;
use crate::components::top_container::TopContainer;
use crate::components::main_container::sticky::Sticky;

#[derive(PartialEq, Properties)]
pub struct UserPageProps {
    pub username: String,
}

#[function_component]
pub fn UserPage(props: &UserPageProps) -> Html {
    let username = props.username.clone();

    let data_state = use_async(async move {
        fetch_user_data_by_username(username).await
    });

    let effect_data_state = data_state.clone();

    use_effect_with(props.username.clone(), move |_| {
        effect_data_state.run();
        || ()
    });
    html! {
        <>
            {
                if let Some(error) = &data_state.error {
                    html! { error }
                } else {
                    html! {}
                }
            }
            {
                if let Some(user_data) = &data_state.data {
                    let user = user_data.get("userByUsername").unwrap().clone();
                    let bio = user.get("bio").unwrap().clone();
                    let about = user_data.get("about").unwrap();
                    let articles_list = user.get("articlesList").unwrap().as_array().unwrap().clone();
                    let recent = user_data.get("recentArticles").unwrap().as_array().unwrap();
                    html! {
                        <>
                            <TopContainer />
                            <main class="container">
                                <div class="row g-5">
                                    <ArticlesList {articles_list} description={bio} />
                                    <Sticky 
                                        about={about.clone()} 
                                        recent={recent.clone()} 
                                        // archives={props.archives.clone()} 
                                        // elsewhere={props.elsewhere.clone()}
                                    />
                                </div>

                            </main>
                        </>

                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}
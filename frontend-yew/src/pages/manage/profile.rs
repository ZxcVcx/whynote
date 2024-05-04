use yew::{function_component, html, Html, Properties};

use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::app::MainRoute;
// use crate::services::use_query::FetchError;
use crate::services::user::fetch_user_data_by_email;
use crate::utils::common::{create_gravatar_url, format_date, shorter_string};
use crate::utils::storage::get_pair_value;

#[derive(PartialEq, Properties)]
pub struct ProfilePageProps {
    
}

#[function_component]
pub fn ProfilePage(props: &ProfilePageProps) -> Html {
    let ProfilePageProps {} = props;
    let navigator = use_navigator().expect("Navigator should be available");
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
                    html! {error}
                } else {
                    html! {}
                }
            }
            {
                if let Some(user_data) = &user_state.data {
                    let user = user_data["userByEmail"].clone();
                    // let banned = user["banned"].clone();
                    let email = user["email"].clone();
                    // let id = user["id"].clone();
                    let username = user["username"].clone();
                    let nickname = user["nickname"].clone();
                    // let blog_name = user["blogName"].clone();
                    let bio = user["bio"].clone();
                    let website = user["website"].clone();
                    let crafts = user["crafts"].as_array().unwrap();
                    let articles = user["articles"].as_array().unwrap();
                    // let carft = user_data["carft"].clone().as_array().unwrap();
                    // let published = user_data["published"].clone().as_array().unwrap();

                    html! {
                        <main class="container">
                            <div class="row g-1">
                                <div class="col-md-3">
                                    // <img class="rounded-circle" src={create_gravatar_url(get_pair_value("email").unwrap().as_str(), 80)} />
                                   
                                    // <div class="d-flex flex-column align-items-center">
                                    <div>
                                        <div class="p-2">
                                            <img class="profile-avatar w-100 img-fluid rounded-circle" src={create_gravatar_url(get_pair_value("email").unwrap().as_str(), 150)} width="150px"/>
                                        </div>
                                        <div class="p-2">
                                            <h4>{nickname.as_str().unwrap().to_string()}</h4>
                                            <p>{username.as_str().unwrap().to_string()}</p>
                                            <p>{bio.as_str().unwrap().to_string()}</p>
                                            <button class="w-100 btn btn-primary">{"Edit Profile"}</button>
                                        </div>
                                        <div class="p-2">
                                            <div>{"Started 2018"}</div>
                                            <div>{email.as_str().unwrap().to_string()}</div>
                                            <a href={website.as_str().unwrap().to_string()}>{website.as_str().unwrap().to_string()}</a>
                                        </div>
                                    </div>
                                </div>
                                <div class="col-md-9">
                                    <div class="p-2">
                                        <h3>{"crafts"}</h3>
                                        {
                                            for crafts.iter().map(|craft| {
                                                let subject = craft["subject"].as_str().unwrap();
                                                let slug = craft["slug"].as_str().unwrap().to_string();
                                                let summary = craft["summary"].as_str().unwrap();
                                                let updated_at = format_date(&craft["updatedAt"], "%b %d, %Y").unwrap();
                                                let category = craft["category"]["name"].as_str().unwrap();

                                                let on_content_click = {
                                                    let navigator = navigator.clone();
                                                    Callback::from(move |_| {
                                                        navigator.push(&MainRoute::ArticlePage {slug: slug.clone() });
                                                    })
                                                };
                                                
                                                html! {
                                                    // <div class="list-group">
                                                    //     <a href={format!("/article/{}", slug)} class="list-group-item list-group-item-action d-flex justify-content-between align-items-center">
                                                    //         {title}
                                                    //     </a>
                                                    // </div>
                                                    <div class="article-card">
                                                        <div class="row g-0">
                                                            <div class="col-md-10">
                                                                <div class="article-card-body">
                                                                    <div class="d-flex justify-content-between align-items-center mb-2">
                                                                        <h6 onclick={on_content_click} class="card-title">{shorter_string(subject, 32)}</h6>
                                                                    </div>
                                                                    // <div onclick={on_content_click} style="cursor: pointer;">
                                                                    <div>

                                                                        // <h6 onclick={on_content_click} class="card-title">{subject}</h6>
                                                                        <p class="article-card-text">{shorter_string(summary, 110)}</p>
                                                                        // <br />
                                                                        <span class="text-muted">{format!{"Updated at {}", updated_at}}
                                                                        </span>
                                                                        <br />
                                                                        <p class="badge bg-secondary">{category}</p>

                                                                    </div>
                                                                </div>
                                                            </div>
                                                            <div class="col-md-2">
                                                                // <div class="d-grid gap-2 d-md-flex justify-content-md-end">
                                                                //             <button class="btn btn-light  me-md-2" type="button">{"Publish"}</button>
                                                                //             <button class="btn btn-light" type="button">{"Edit"}</button>
                                                                //             <button class="btn btn-light" type="button">{"Delete"}</button>

                                                                // </div>
                                                                <div class="dropdown d-grid gap-2 d-md-flex justify-content-md-end">
                                                                    <p class="btn btn-light" type="button" data-bs-toggle="dropdown" aria-expanded="false">{". . ."}</p>
                                                                    <ul class="dropdown-menu">
                                                                        <li><a class="dropdown-item" href="#">{"Publish"}</a></li>
                                                                        <li><a class="dropdown-item" href="#">{"Edit"}</a></li>
                                                                        <li><a class="dropdown-item" href="#">{"Delete"}</a></li>
                                                                    </ul>
                                                                </div>
                                                                // <img src="river-crossing.jpg" class="img-fluid rounded-end card-img-right" alt="...">
                                                            </div>
                                                        </div>
                                                    </div>
                                                }
                                            })
                                        }
                                    </div>
                                    <div class="p-2">
                                        <h3>{"articles"}</h3>
                                        {
                                            for articles.iter().map(|article| {
                                                let subject = article["subject"].as_str().unwrap();
                                                let slug = article["slug"].as_str().unwrap().to_string();
                                                let updated_at = format_date(&article["updatedAt"], "%b %d, %Y").unwrap();
                                                let category = article["category"]["name"].as_str().unwrap();
                                                let summary = article["summary"].as_str().unwrap();

                                                let on_content_click = {
                                                    let navigator = navigator.clone();
                                                    Callback::from(move |_| {
                                                        navigator.push(&MainRoute::ArticlePage {slug: slug.clone() });
                                                    })
                                                };
                                                
                                                html! {
                                                    // <div class="list-group">
                                                    //     <a href={format!("/article/{}", slug)} class="list-group-item list-group-item-action d-flex justify-content-between align-items-center">
                                                    //         {title}
                                                    //     </a>
                                                    // </div>
                                                    <div class="article-card">
                                                        <div class="row g-0">
                                                            <div class="col-md-10">
                                                                <div class="article-card-body">
                                                                    <div class="d-flex justify-content-between align-items-center mb-2">
                                                                        <h6 onclick={on_content_click} class="card-title">{shorter_string(subject, 32)}</h6>

                                                                    </div>
                                                                    // <div onclick={on_content_click} style="cursor: pointer;">
                                                                    <div>
                                                                        // <h6 onclick={on_content_click} class="card-title">{subject}</h6>
                                                                        <p class="article-card-text">{shorter_string(summary, 128)}</p>
                                                                        // <br />
                                                                        <span class="text-muted">{format!{"Updated at {}", updated_at}}
                                                                        </span>
                                                                        <br />
                                                                        <p class="badge bg-secondary">{category}</p>

                                                                    </div>
                                                                </div>
                                                            </div>
                                                            <div class="col-md-2">
                                                                // <div class="d-grid gap-2 d-md-flex justify-content-md-end">
                                                                //             <button class="btn btn-light me-md-2" type="button">{"Edit"}</button>
                                                                //             <button class="btn btn-light" type="button">{"Delete"}</button>
                                                                // </div>
                                                                // <img src="river-crossing.jpg" class="img-fluid rounded-end card-img-right" alt="...">
                                                                <div class="dropdown d-grid gap-2 d-md-flex justify-content-md-end">
                                                                    <p class="btn btn-light" type="button" data-bs-toggle="dropdown" aria-expanded="false">{". . ."}</p>
                                                                    <ul class="dropdown-menu">
                                                                        // <li><a class="dropdown-item" href="#">{"Publish"}</a></li>
                                                                        <li><a class="dropdown-item" href="#">{"Edit"}</a></li>
                                                                        <li><a class="dropdown-item" href="#">{"Delete"}</a></li>
                                                                    </ul>
                                                                </div>
                                                            </div>
                                                            
                                                            // <div class="col-md-2">
                                                            //     <img src="river-crossing.jpg" class="img-fluid rounded-end card-img-right" alt="...">
                                                            // </div>
                                                        </div>
                                                    </div>
                                                }
                                            })
                                        }
                                    </div>
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
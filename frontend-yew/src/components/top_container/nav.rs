use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;
use crate::{app::MainRoute, services::categories::fetch_categories_list};


// use crate::app::AppRoute;

/// Nav component
#[function_component]
pub fn Nav() -> Html {
    let nav_list =
        use_async(async move {
            fetch_categories_list().await
        });

    let effect_nav_list = nav_list.clone();

    use_effect_with((), move |_| {
        effect_nav_list.run();
        || ()
    });

    // let nav_vec = nav_list.data.clone().unwrap().get("cetegories").unwrap().as_array().unwrap();


    html! {
        
        // <div class="container py-4">
        //     <nav class="flex space-x-4 items-center">
        //         <Link<AppRoute> to={AppRoute::Home} classes="text-emerald-800 underline" >
        //             <img class="w-10 h-10" src="logo.svg" alt="Yew" />
        //         </Link<AppRoute>>
        //         <Link<AppRoute> to={AppRoute::Home} classes="text-emerald-800 underline" >{ "Home" }</Link<AppRoute>>
        //         <Link<AppRoute> to={AppRoute::About} classes="text-emerald-800 underline">{ "About" }</Link<AppRoute>>
        //     </nav>
        // </div>

        <div class="nav-scroller py-1 mb-3 border-bottom">
            <nav class="nav nav-underline justify-content-between">
                <Link<MainRoute> classes="nav-item nav-link link-body-emphasis" to={MainRoute::Home}>
                    { "Home" }
                </Link<MainRoute>>
                // <Link<AppRoute> to={AppRoute::Editor} classes="nav-item nav-link link-body-emphasis">
                //     { "Editor" }
                // </Link<AppRoute>>

                {
                    if nav_list.loading {
                        html! { "Loading" }
                    } else {
                        html! {}
                    }
                }
                // https://yew.rs/docs/concepts/router#function-components
                {
                    if let Some(data) = &nav_list.data {
                        let nav_vec = data.get("categories").unwrap().as_array().unwrap();
                        nav_vec.into_iter().map(|category| {
                            if category.get("articles").unwrap().as_array().unwrap().len() > 0 {
                                // let uri = category.get("uri").unwrap().to_string();
                                html! {
                                    // <a class="nav-item nav-link link-body-emphasis" href="#">{category["name"].as_str()}</a>
                                    <Link<MainRoute> classes="nav-item nav-link link-body-emphasis" to={MainRoute::CategoryPage {slug: category["slug"].as_str().unwrap().to_string() }}>
                                        { category["name"].as_str() }
                                    </Link<MainRoute>>
                                }
                            } else {
                                html! {}
                            }
                        }).collect::<Html>()
                    } else {
                        html! {}
                    }
                }
                // <a class="nav-item nav-link link-body-emphasis active" href="#">{"World"}</a>
                // <a class="nav-item nav-link link-body-emphasis" href="#">{"U.S."}</a>
                // <a class="nav-item nav-link link-body-emphasis" href="#">{"Technology"}</a>
                // <a class="nav-item nav-link link-body-emphasis" href="#">{"Design"}</a>
                // <a class="nav-item nav-link link-body-emphasis" href="#">{"Culture"}</a>
                // <a class="nav-item nav-link link-body-emphasis" href="#">{"Business"}</a>
                // <a class="nav-item nav-link link-body-emphasis" href="#">{"Politics"}</a>
                // <a class="nav-item nav-link link-body-emphasis" href="#">{"Opinion"}</a>
                // <a class="nav-item nav-link link-body-emphasis" href="#">{"Science"}</a>
                // <a class="nav-item nav-link link-body-emphasis" href="#">{"Health"}</a>
                // <a class="nav-item nav-link link-body-emphasis" href="#">{"Style"}</a>
                // <a class="nav-item nav-link link-body-emphasis" href="#">{"Travel"}</a>
            </nav>
        </div>
    }
}

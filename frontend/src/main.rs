use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod services;

mod pages;
use pages::{home::Home, projects::Projects, users::Users};

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/users")]
    Users,
}

// https://yew.rs/docs/concepts/router
// https://docs.rs/yew-router/latest/yew_router/
// https://github.com/martinschultzkristensen/yew-web-app/

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
           <Home />
        },
        Route::Projects => html! {
            <Projects />
        },
        Route::Users => html! {
            <Users />
        },
    }
}

#[function_component(NavItems)]
pub fn nav_items() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
            <button {onclick}>{"click to go home"}</button>
        }
    };

    // let go_to_first_post_button = {
    //     let navigator = navigator.clone();
    //     let onclick = Callback::from(move |_| navigator.push(&Route::Post { id: "first-post".to_string() }));
    //     html! {
    //         <button {onclick}>{"click to go the first post"}</button>
    //     }
    // };

    let go_to_projects_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Projects));
        html! {
            <button {onclick}>{"click to go to secure"}</button>
        }
    };

    let go_to_users_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Users));
        html! {
            <button {onclick}>{"click to go to users"}</button>
        }
    };

    // https://yew.rs/docs/next/concepts/basic-web-technologies/css
    html! {
        <div>
            {go_home_button}
            {go_to_projects_button}
            {go_to_users_button}
        </div>
    }
}

#[function_component(Main)]
pub fn app() -> Html {
    // should not contain a parameter
    html! {
        <BrowserRouter>
            <div class="logo-title">
                { "tide-async-graphql-mongodb / frontend-yew" }
            </div>
            <NavItems></NavItems>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}

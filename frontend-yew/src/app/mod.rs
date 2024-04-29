use yew::prelude::*;
use yew_router::prelude::*;

pub mod about;
pub mod home;

// use crate::components::nav::Nav;
use crate::components::top_container::TopContainer;
use about::About;
use home::Home;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/about")]
    About,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/category/:slug")]
    Category { slug: String},
    #[at("/")]
    Home,
}

/// Switch app routes
pub fn switch(routes: AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::PageNotFound => html! { "Page not found" },
        AppRoute::Category { slug } => html! { format!("Category: {}", slug) },
    }
}

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            // <div class="flex min-h-screen flex-col">
            <div>
                // <Nav />
                <TopContainer />
                <Switch<AppRoute> render={switch} />
            </div>
        </HashRouter>
    }
}

use yew::prelude::*;
use yew_router::prelude::*;

pub mod about;
pub mod home;

// use crate::components::nav::Nav;
use about::About;
use home::Home;

use crate::pages::signin::SignIn;
use crate::pages::category::CategoryPage;
use crate::pages::user::UserPage;
use crate::components::bottom_container::footer::Footer;
use crate::pages::article::ArticlePage;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/manage")]
    ManageRoot,
    #[at("/manage/*")]
    Manage,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/category/:slug")]
    CategoryPage { slug: String},
    #[at("/user/:username")]
    UserPage { username: String },
    #[at("/article/:slug")]
    ArticlePage { slug: String },

}

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum ManageRoute {
    #[at("/manage")]
    Profile,
    #[at("/manage/signup")]
    SignUp,
    #[at("/manage/signin")]
    SignIn,
    #[not_found]
    #[at("/manage/404")]
    NotFound,
}

/// Switch app routes
pub fn switch_main(routes: MainRoute) -> Html {
    match routes.clone() {
        MainRoute::Home => html! { <Home /> },
        MainRoute::About => html! { <About /> },
        // MainRoute::ManageRoot => html! { <Redirect<ManageRoute> to={ManageRoute::Profile} /> },
        MainRoute::ManageRoot | MainRoute::Manage => html! { <Switch<ManageRoute> render={switch_settings} /> },
        MainRoute::PageNotFound => html! { "Page not found" },
        // MainRoute::CategoryPage { slug } => html! { format!("Category: {}", slug) },
        MainRoute::CategoryPage { slug } => html! { <CategoryPage {slug} /> },
        MainRoute::UserPage { username } => html! { <UserPage {username} /> },
        MainRoute::ArticlePage { slug } => html! { <ArticlePage {slug} /> },
    }
}

fn switch_settings(routes: ManageRoute) -> Html {
    match routes.clone() {
        ManageRoute::Profile => html! {<h1>{"Profile"}</h1>},
        ManageRoute::SignUp => html! {<h1>{"signup"}</h1>},
        ManageRoute::SignIn => html! { <SignIn /> },
        ManageRoute::NotFound => html! {<Redirect<MainRoute> to={MainRoute::PageNotFound}/>}
    }
}

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        // <HashRouter>
        <BrowserRouter>
            // <div class="flex min-h-screen flex-col">
            <>
                // <Nav />
                <Switch<MainRoute> render={switch_main} />
                <Footer />
            </>
        </BrowserRouter>
        // </HashRouter>
    }
}

use yew::prelude::*;
use yew_router::prelude::*;

pub mod home;

// use crate::components::nav::Nav;
// use about::About;
use home::Home;

use crate::components::bottom_container::footer::Footer;
use crate::components::theme_toggle::ThemeToggle;
use crate::pages::common::article::ArticlePage;
use crate::pages::common::category::CategoryPage;
use crate::pages::common::search::SearchPage;
use crate::pages::common::test::Test;
use crate::pages::common::user::UserPage;
use crate::pages::manage::categories::CategoryManagePage;
use crate::pages::manage::editor::EditorPage;
use crate::pages::manage::init::InitPage;
use crate::pages::manage::new_editor::NewEditorPage;
use crate::pages::manage::profile::ProfilePage;
use crate::pages::manage::signin::SignIn;
use crate::pages::manage::topics::TopicManagePage;
use crate::utils::common::is_logged_in;

#[derive(Properties, PartialEq)]
pub struct RouteGuardProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(RouteGuard)]
fn route_guard(props: &RouteGuardProps) -> Html {
    if is_logged_in() {
        html! { for props.children.iter() }
    } else {
        html! { <Redirect<MainRoute> to={MainRoute::Home} /> }
    }
}

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/test")]
    Test,
    #[at("/manage")]
    ManageRoot,
    #[at("/manage/*")]
    Manage,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/category/:slug")]
    CategoryPage { slug: String },
    #[at("/user/:username")]
    UserPage { username: String },
    #[at("/article/:slug")]
    ArticlePage { slug: String },
    #[at("/search")]
    SearchPage,
    #[at("/signin")]
    SignIn,
}

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum ManageRoute {
    #[at("/manage")]
    Manage,
    #[at("/manage/profile")]
    Profile,
    #[at("/manage/init")]
    Init,
    // #[at("/manage/signin")]
    // SignIn,
    #[not_found]
    #[at("/manage/404")]
    NotFound,
    #[at("/manage/content")]
    Content,
    #[at("/manage/editor/article/:id")]
    Editor { id: String },
    #[at("/manage/editor/new")]
    NewEditor,
    #[at("/manage/categories")]
    Categories,
    #[at("/manage/topics")]
    Topics,
}

/// Switch app routes
pub fn switch_main(routes: MainRoute) -> Html {
    match routes.clone() {
        MainRoute::Home => html! { <Home /> },
        // MainRoute::About => html! { <About /> },
        // MainRoute::ManageRoot => html! { <Redirect<ManageRoute> to={ManageRoute::Profile} /> },
        MainRoute::ManageRoot | MainRoute::Manage => {
            // html! { <Switch<ManageRoute> render={switch_settings} /> }
            html! {
                <RouteGuard>
                    <Switch<ManageRoute> render={switch_settings} />
                </RouteGuard>
            }
        }
        MainRoute::PageNotFound => html! { "Page not found" },
        // MainRoute::CategoryPage { slug } => html! { format!("Category: {}", slug) },
        MainRoute::CategoryPage { slug } => html! { <CategoryPage {slug} /> },
        MainRoute::UserPage { username } => html! { <UserPage {username} /> },
        MainRoute::ArticlePage { slug } => html! { <ArticlePage {slug} /> },
        MainRoute::Test => html! { <Test /> },
        MainRoute::SearchPage => html! { <SearchPage /> },
        MainRoute::SignIn => html! { <SignIn /> },
    }
}

fn switch_settings(routes: ManageRoute) -> Html {
    match routes.clone() {
        ManageRoute::Manage => html! {<h1>{"Manage"}</h1>},
        ManageRoute::Profile => html! { <ProfilePage /> },
        ManageRoute::Init => html! { <InitPage /> },
        // ManageRoute::SignIn => html! { <SignIn /> },
        ManageRoute::NotFound => html! {<Redirect<MainRoute> to={MainRoute::PageNotFound}/>},
        ManageRoute::Content => html! {<h1>{"Content"}</h1>},
        ManageRoute::Editor { id } => html! { <EditorPage {id} /> },
        ManageRoute::NewEditor => html! { <NewEditorPage />},
        ManageRoute::Categories => html! { <CategoryManagePage /> },
        ManageRoute::Topics => html! { <TopicManagePage /> },
    }
}

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        // <HashRouter>
        <BrowserRouter>
            <>
                <ThemeToggle />
                <Switch<MainRoute> render={switch_main} />
                <Footer />
            </>
        </BrowserRouter>
        // </HashRouter>
    }
}

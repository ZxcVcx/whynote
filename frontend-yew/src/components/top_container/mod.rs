pub mod header;
pub mod nav;
// pub mod top_container;

use header::Header;
use nav::Nav;
use yew::prelude::*;
/// Nav component
#[function_component(TopContainer)]
pub fn top_container() -> Html {
    html! {
        <div class="container py-4">
            <Header />
            <Nav />
        </div>
    }
}

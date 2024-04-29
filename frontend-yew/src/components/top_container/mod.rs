pub mod nav;
pub mod header;
// pub mod top_container;

use yew::prelude::*;
use nav::Nav;
use header::Header;
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

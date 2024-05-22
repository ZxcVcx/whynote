pub mod header;
pub mod manage_top_container;
pub mod nav;
pub mod user_drop_down;
// pub mod top_container;

use header::Header;
use nav::Nav;
use yew::prelude::*;

use crate::utils::common::is_logged_in;
/// Nav component
#[function_component(TopContainer)]
pub fn top_container() -> Html {
    let login_state = use_state(|| is_logged_in());
    html! {
        <div class="container py-4">
            <Header {login_state}/>
            <Nav />
        </div>
    }
}

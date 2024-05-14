use yew::{function_component, html, Callback, Html, Properties};

use crate::components::bottom_container::load_js::LoadJs;

#[derive(PartialEq, Properties)]
pub struct FooterProps {}

#[function_component]
pub fn Footer(props: &FooterProps) -> Html {
    let FooterProps {} = props;
    let back_to_top = {
        Callback::from(move |_| {
            gloo_utils::document_element().set_scroll_top(0);
        })
    };

    html! {
        <>
            <footer class="footer py-5 text-center text-body-secondary bg-body-tertiary">
                // <p>{"Blog built for <a href="https://getbootstrap.com/">WhyNote</a> by <a
                //     href="https://twitter.com/mdo">Nathan Wang</a>."}</p>
                <p>{"© 2024 Blog build for WhyNote by Nathan Wang."}</p>
                <p class="mb-0">
                // <a onclick={back_to_top} href="" >{"Back to top"}</a>
                <button class="btn btn-outline-secondary" onclick={back_to_top}>{"Back to top"}</button>
                </p>
            </footer>
            <LoadJs />
        </>
    }
}

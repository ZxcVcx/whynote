use yew::{function_component, html, Html, Properties};

use crate::components::bottom_container::load_js::LoadJs;

#[derive(PartialEq, Properties)]
pub struct FooterProps {}

#[function_component]
pub fn Footer(props: &FooterProps) -> Html {
    let FooterProps {} = props;
    html! {
        <>
            <footer class="py-5 text-center text-body-secondary bg-body-tertiary">
                // <p>{"Blog built for <a href="https://getbootstrap.com/">WhyNote</a> by <a
                //     href="https://twitter.com/mdo">Nathan Wang</a>."}</p>
                <p>{"© 2024 Blog build for WhyNote by Nathan Wang."}</p>
                <p class="mb-0">
                <a href="#">{"Back to top"}</a>
                </p>
            </footer>
            <LoadJs />
        </>
    }
}

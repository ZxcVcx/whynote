use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
use yew_hooks::prelude::*;
// use yew_router::prelude::*;

use crate::components::manage_container::editor::Editor;
use crate::services::article::fetch_article_data_by_id;

#[derive(PartialEq, Properties)]
pub struct EditorPageProps {
    pub id: String,
}

#[function_component]
pub fn EditorPage(props: &EditorPageProps) -> Html {
    let id = props.id.clone();
    let article_state = use_async(async { fetch_article_data_by_id(id).await });

    let effect_article_state = article_state.clone();
    use_effect_with(props.id.clone(), move |_| {
        effect_article_state.run();
        || ()
    });
    html! {
        <>
            {
                if article_state.loading {
                    html! { "Loading" }
                } else {
                    html! {}
                }
            }
            {
                if let Some(error) = &article_state.error {
                    html! {error}
                } else {
                    html! {}
                }
            }
            {
                if let Some(article_data) = &article_state.data {
                    let article = article_data["articleById"].clone();
                    html! {
                        <Editor {article} />
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}

use yew::prelude::*;
use yew_hooks::prelude::*;

use yew::{function_component, html, Html, Properties};

use crate::components::main_container::articles_list::ArticlesList;
use crate::components::main_container::sticky::Sticky;
use crate::components::top_container::TopContainer;
use crate::services::categories::fetch_category_data_by_slug;

#[derive(PartialEq, Properties)]
pub struct CategoryPageProps {
    pub slug: String,
}

#[function_component]
pub fn CategoryPage(props: &CategoryPageProps) -> Html {
    // let CategoryPageProps { slug } = props;
    let slug = props.slug.clone();

    let data_state = use_async(async move { fetch_category_data_by_slug(slug).await });

    let effect_data_state = data_state.clone();

    use_effect_with(props.slug.clone(), move |_| {
        effect_data_state.run();
        || ()
    });

    html! {
        <>
            {
                if let Some(error) = &data_state.error {
                    html! { error }
                } else {
                    html! {}
                }
            }

            {
                if data_state.loading {
                    html! {
                        <div class="container py-4 position-fixed">
                            <div class="spinner-border text-primary" role="status">
                                <span class="visually-hidden">{ "Loading..." }</span>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            {
                if let Some(category_data) = &data_state.data {
                    let category = category_data.get("categoryBySlug").unwrap().clone();
                    // let name = category.get("name").unwrap().as_str().unwrap();
                    let description = category.get("description").unwrap().clone();
                    let about = category_data.get("about").unwrap();
                    let articles_list = category.get("articlesList").unwrap().as_array().unwrap().clone();
                    let recent = category_data.get("recentArticles").unwrap().as_array().unwrap();
                    html! {
                        <>
                            <TopContainer />
                            <main class="container">
                                <div class="row g-5">
                                    <ArticlesList {articles_list} {description} />
                                    <Sticky
                                        about={about.clone()}
                                        recent={recent.clone()}
                                        // archives={props.archives.clone()}
                                        // elsewhere={props.elsewhere.clone()}
                                    />
                                </div>

                            </main>
                        </>

                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}

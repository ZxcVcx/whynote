use yew::{function_component, html, Html, Properties};
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::services::article::fetch_article_data_by_slug;
use crate::components::top_container::TopContainer;
use crate::components::main_container::article::Article;
use crate::components::main_container::sticky::Sticky;

#[derive(PartialEq, Properties)]
pub struct ArticlePageProps {
    pub slug: String,
}

#[function_component]
pub fn ArticlePage(props: &ArticlePageProps) -> Html {
    // let ArticlePageProps {slug} = props;
    let slug = props.slug.clone();
    
    let article_state = use_async(async move {
        fetch_article_data_by_slug(slug).await
    });

    let effect_article_state = article_state.clone();

    use_effect_with(props.slug.clone(), move |_| {
        effect_article_state.run();
        || ()
    });

    html! {
        <>
        // {
        //     if article_state.loading {
        //         html! { <div>{ "Loading..." }</div>}
        //     } else {
        //         html! {}
        //     }
        // }
            {
                if let Some(error) = &article_state.error {
                    html! { error }
                } else {
                    html! {}
                }
            }
            {
                if let Some(article_data) = &article_state.data {
                    let article = article_data.get("articleBySlug").unwrap().clone();
                    let about = article_data.get("about").unwrap();
                    let recent = article_data.get("recentArticles").unwrap().as_array().unwrap();
                    let nickname = article["user"]["nickname"].as_str().unwrap();
                    html! {
                        <>
                            <TopContainer />
                            <main class="container">
                                <div class="row g-5">
                                    <div class="col-md-8">
                                        <h3 class="pb-4 mb-4 fst-italic border-bottom">
                                            { format!("From {}", nickname).to_string() }
                                        </h3>
                                        <Article {article} />
                                    </div>
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
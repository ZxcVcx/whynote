use crate::components::main_container::article::Article;
use crate::components::main_container::sticky::Sticky;
use crate::components::top_container::TopContainer;
use crate::services::article::fetch_article_data_by_slug;
use crate::utils::constants::CFG;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
use yew_hooks::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ArticlePageProps {
    pub slug: String,
}

#[function_component]
pub fn ArticlePage(props: &ArticlePageProps) -> Html {
    let slug = props.slug.clone();
    let container_ref = use_node_ref();

    let article_state = use_async(async move { fetch_article_data_by_slug(slug).await });

    let effect_article_state = article_state.clone();

    let ref_clone = container_ref.clone();

    let load_giscus_widget = || {
        if let Some(container) = ref_clone.cast::<web_sys::HtmlDivElement>() {
            container.set_inner_html("");

            let widget = gloo_utils::document()
                .create_element("giscus-widget")
                .unwrap();
            widget.set_attribute("id", "comments").unwrap();
            widget
                .set_attribute("repo", CFG.get("COMMENT_REPO").unwrap().as_str())
                .unwrap();
            widget
                .set_attribute("repoid", CFG.get("COMMENT_REPO_ID").unwrap().as_str())
                .unwrap();
            widget
                .set_attribute("category", CFG.get("COMMENT_CATEGORY").unwrap().as_str())
                .unwrap();
            widget
                .set_attribute(
                    "categoryid",
                    CFG.get("COMMENT_CATEGORY_ID").unwrap().as_str(),
                )
                .unwrap();
            widget
                .set_attribute("mapping", CFG.get("COMMENT_MAPPING").unwrap().as_str())
                .unwrap();
            widget
                .set_attribute("strict", CFG.get("COMMENT_STRICT").unwrap().as_str())
                .unwrap();
            widget
                .set_attribute(
                    "reactionsenabled",
                    CFG.get("COMMENT_REACTIONS_ENABLED").unwrap().as_str(),
                )
                .unwrap();
            widget
                .set_attribute(
                    "emitmetadata",
                    CFG.get("COMMENT_EMIT_METADATA").unwrap().as_str(),
                )
                .unwrap();
            widget
                .set_attribute(
                    "inputposition",
                    CFG.get("COMMENT_INPUT_POSITION").unwrap().as_str(),
                )
                .unwrap();
            widget
                .set_attribute("theme", CFG.get("COMMENT_THEME").unwrap().as_str())
                .unwrap();
            widget
                .set_attribute("lang", CFG.get("COMMENT_LANG").unwrap().as_str())
                .unwrap();
            widget
                .set_attribute("loading", CFG.get("COMMENT_LOADING").unwrap().as_str())
                .unwrap();

            container.append_child(&widget).unwrap();
        }
    };

    use_effect_with(props.slug.clone(), move |_| {
        effect_article_state.run();
        || ()
    });

    html! {
        <>
            {
                if article_state.loading {
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
                                    <div id="article-container" class="col-md-8 article-container">
                                        <h3 class="pb-4 mb-4 fst-italic border-bottom">
                                            { format!("From {}", nickname).to_string() }
                                        </h3>
                                        <Article {article} />
                                        // <GiscusWidget slug={props.slug.clone()}/>
                                        <div ref={container_ref}>
                                            <giscus-widget
                                                id="comments"
                                                repo={CFG.get("COMMENT_REPO").unwrap().clone()}
                                                repoid={CFG.get("COMMENT_REPO_ID").unwrap().clone()}
                                                category={CFG.get("COMMENT_CATEGORY").unwrap().clone()}
                                                categoryid={CFG.get("COMMENT_CATEGORY_ID").unwrap().clone()}
                                                mapping={CFG.get("COMMENT_MAPPING").unwrap().clone()}
                                                strict={CFG.get("COMMENT_STRICT").unwrap().clone()}
                                                reactionsenabled={CFG.get("COMMENT_REACTIONS_ENABLED").unwrap().clone()}
                                                emitmetadata={CFG.get("COMMENT_EMIT_METADATA").unwrap().clone()}
                                                inputposition={CFG.get("COMMENT_INPUT_POSITION").unwrap().clone()}
                                                theme={CFG.get("COMMENT_THEME").unwrap().clone()}
                                                lang={CFG.get("COMMENT_LANG").unwrap().clone()}
                                                loading={CFG.get("COMMENT_LOADING").unwrap().clone()}
                                            ></giscus-widget>
                                        </div>
                                        {
                                            {
                                                load_giscus_widget();
                                                html! {}
                                            }
                                        }
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

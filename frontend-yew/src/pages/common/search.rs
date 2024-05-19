use crate::app::MainRoute;
use crate::components::top_container::TopContainer;
use crate::services::search::{fetch_articles_data_as_vec, SearchData};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SearchPageProps {}

#[function_component]
pub fn SearchPage(props: &SearchPageProps) -> Html {
    let SearchPageProps {} = props;
    let articles = use_state(|| vec![]);
    let search_query = use_state(|| "".to_string());

    {
        let articles = articles.clone();
        use_effect_with((), move |_| {
            let articles = articles.clone();
            spawn_local(async move {
                let fetched_articles = fetch_articles_data_as_vec()
                    .await
                    .expect("Failed to fetch articles");
                articles.set(fetched_articles);
            });
            || ()
        });
    }

    let on_search_input = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let filtered_articles: Vec<SearchData> = articles
        .iter()
        .cloned()
        .filter(|article| {
            article
                .subject
                .to_lowercase()
                .contains(&search_query.to_lowercase())
                || article
                    .content
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
                || article
                    .slug
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
        })
        .collect();

    html! {
        <>
        <TopContainer />
        <div>
            <input type="text" placeholder="搜索文章" oninput={on_search_input} />
            <ul>
                { for filtered_articles.iter().map(|article| html! {
                    <li key={article.id.clone()}>
                        <h2>{ &article.subject }</h2>
                        <p>{ &article.summary }</p>
                        <Link<MainRoute> to={MainRoute::ArticlePage {slug: article.slug.clone() }}>
                        {"Read more"}
                        </Link<MainRoute>>
                    </li>
                })}
            </ul>
        </div>
        </>
    }
}

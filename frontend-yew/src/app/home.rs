use crate::{pages::home::HomePage, services::articles::fetch_home_data};
use yew::prelude::*;
use yew_hooks::prelude::*;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let home_state = use_async(async { fetch_home_data().await });

    let effect_home_state = home_state.clone();

    // let emphasis = use_state(|| vec![]);
    // let secondary = use_state(|| vec![]);
    // let recommand = use_state(|| vec![]);

    use_effect_with((), move |_| {
        effect_home_state.run();
        || ()
    });

    html! {
        <>
            {
                if let Some(error) = &home_state.error {
                    html! { error }
                } else {
                    html! {}

                }
            }
            {
                if let Some(home_data) = &home_state.data {
                    let top_articles = home_data.get("topArticles").unwrap().as_array().unwrap();
                    let emphasis = top_articles[0].clone();
                    let secondary = vec![top_articles[1].clone(), top_articles[2].clone()];
                    let recommanded = home_data.get("recommendedArticles").unwrap().as_array().unwrap().clone();
                    let about = home_data.get("about").unwrap().clone();
                    let recent = home_data.get("recentArticles").unwrap().as_array().unwrap().clone();

                    html! {
                        <HomePage
                            emphasis={emphasis}
                            secondary={secondary}
                            recommanded={recommanded}
                            about={about}
                            recent={recent}
                        />
                    }
                } else {
                    html! {}
                }
            }
        </>

        // <div class="container text-center">
        //     <header class="space-y-8">
        //         <p>
        //             <a href="https://yew.rs" target="_blank" rel="noopener noreferrer">
        //                 <img class="w-48 h-48 mx-auto mt-24" src="logo.svg" alt="Yew" />
        //             </a>
        //         </p>
        //         <p>
        //             <a id="learn_yew" class="text-emerald-800 underline" href="https://yew.rs" target="_blank" rel="noopener noreferrer">{ "Learn Yew" }</a>
        //         </p>
        //         <p class="space-x-4">
        //             <button class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 bg-emerald-600 text-slate-100 hover:bg-emerald-600/90" onclick={ondecrease}>{ "Decrease" }</button>
        //             <span class="w-12 inline-block">{ *counter }</span>
        //             <button class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 bg-emerald-600 text-slate-100 hover:bg-emerald-600/90" onclick={onincrease}>{ "Increase" }</button>
        //         </p>
        //         <p>
        //             { "Edit " } <code>{ "src/app/home.rs" }</code> { " and save to reload." }
        //         </p>
        //     </header>
        // </div>
    }
}

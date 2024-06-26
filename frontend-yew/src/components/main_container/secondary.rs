use serde_json::Value;
use yew::{function_component, html, Html, Properties};
use yew_router::prelude::*;

use crate::app::MainRoute;
use crate::utils::common::format_date;

#[derive(PartialEq, Properties)]
pub struct SecondaryProps {
    pub secondary: Vec<Value>,
}

#[function_component]
pub fn Secondary(props: &SecondaryProps) -> Html {
    let secondary = props.secondary.clone();
    html! {
        <div class="row mb-2">
            {
                secondary.into_iter().map(|article| {
                    let title = article.get("subject").unwrap().as_str().unwrap();
                    let summary = article.get("summary").unwrap().as_str().unwrap();
                    let category = article.get("category").unwrap().get("name").unwrap().as_str().unwrap();
                    // let updated_at = article.get("updatedAt").unwrap().as_str().unwrap();
                    let updated_at = format_date(article.get("updatedAt").unwrap(), "%b, %d").unwrap();
                    let slug = article.get("slug").unwrap().as_str().unwrap().to_string().clone();
                    html! {
                        <div class="col-md-6">
                            <div class="row g-0 border rounded overflow-hidden flex-md-row mb-4 shadow-sm h-md-250 position-relative">
                                <div class="col p-4 d-flex flex-column position-static">
                                    <strong class="d-inline-block mb-2 text-primary-emphasis">{category}</strong>
                                    <h2 class="mb-0">{title}</h2>
                                    <div class="mb-1 text-body-secondary">{updated_at}</div>
                                    <p class="card-text mb-auto">{summary}</p>
                                    <Link<MainRoute> classes="icon-link gap-1 icon-link-hover stretched-link" to={MainRoute::ArticlePage {slug: slug.clone() }}>
                                        {"Continue reading..."}
                                    </Link<MainRoute>>
                                    // <a href="#" class="icon-link gap-1 icon-link-hover stretched-link">
                                    //     {"Continue reading"}
                                    //     // <svg class="bi">
                                    //     //     <use xlink:true xlink:href="#chevron-right" />
                                    //     // </svg>

                                    // </a>
                                </div>
                                <div class="col-auto d-none d-lg-block">
                                    <svg class="bd-placeholder-img" width="200" height="250" xmlns="http://www.w3.org/2000/svg" role="img"
                                        aria-label="Placeholder: Thumbnail" preserveAspectRatio="xMidYMid slice" focusable="false">
                                        <title>{"Placeholder"}</title>
                                        <rect width="100%" height="100%" fill="#55595c" /><text x="50%" y="50%" fill="#eceeef"
                                            dy=".3em">{"Thumbnail"}</text>
                                    </svg>
                                </div>
                            </div>
                        </div>
                    }
                }).collect::<Html>()
            }
        //   <div class="col-md-6">
        //     <div class="row g-0 border rounded overflow-hidden flex-md-row mb-4 shadow-sm h-md-250 position-relative">
        //       <div class="col p-4 d-flex flex-column position-static">
        //         <strong class="d-inline-block mb-2 text-primary-emphasis">World</strong>
        //         <h3 class="mb-0">Featured post</h3>
        //         <div class="mb-1 text-body-secondary">Nov 12</div>
        //         <p class="card-text mb-auto">This is a wider card with supporting text below as a natural lead-in to
        //           additional content.</p>
        //         <a href="#" class="icon-link gap-1 icon-link-hover stretched-link">
        //           Continue reading
        //           <svg class="bi">
        //             <use xlink:href="#chevron-right" />
        //           </svg>
        //         </a>
        //       </div>
        //       <div class="col-auto d-none d-lg-block">
        //         <svg class="bd-placeholder-img" width="200" height="250" xmlns="http://www.w3.org/2000/svg" role="img"
        //           aria-label="Placeholder: Thumbnail" preserveAspectRatio="xMidYMid slice" focusable="false">
        //           <title>Placeholder</title>
        //           <rect width="100%" height="100%" fill="#55595c" /><text x="50%" y="50%" fill="#eceeef"
        //             dy=".3em">Thumbnail</text>
        //         </svg>
        //       </div>
        //     </div>
        //   </div>
        //   <div class="col-md-6">
        //     <div class="row g-0 border rounded overflow-hidden flex-md-row mb-4 shadow-sm h-md-250 position-relative">
        //       <div class="col p-4 d-flex flex-column position-static">
        //         <strong class="d-inline-block mb-2 text-success-emphasis">Design</strong>
        //         <h3 class="mb-0">Post title</h3>
        //         <div class="mb-1 text-body-secondary">Nov 11</div>
        //         <p class="mb-auto">This is a wider card with supporting text below as a natural lead-in to additional
        //           content.</p>
        //         <a href="#" class="icon-link gap-1 icon-link-hover stretched-link">
        //           Continue reading
        //           <svg class="bi">
        //             <use xlink:href="#chevron-right" />
        //           </svg>
        //         </a>
        //       </div>
        //       <div class="col-auto d-none d-lg-block">
        //         <svg class="bd-placeholder-img" width="200" height="250" xmlns="http://www.w3.org/2000/svg" role="img"
        //           aria-label="Placeholder: Thumbnail" preserveAspectRatio="xMidYMid slice" focusable="false">
        //           <title>Placeholder</title>
        //           <rect width="100%" height="100%" fill="#55595c" /><text x="50%" y="50%" fill="#eceeef"
        //             dy=".3em">Thumbnail</text>
        //         </svg>
        //       </div>
        //     </div>
        //   </div>
        </div>

    }
}

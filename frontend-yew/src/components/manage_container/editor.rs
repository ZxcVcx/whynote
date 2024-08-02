use serde_json::Value;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
// use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
use yew_router::prelude::*;

use crate::app::ManageRoute;
use crate::services::article::{delete_article_data, new_article_data, update_article_data};
use crate::services::categories::fetch_categories_list;
use crate::utils::storage::get_pair_value;

#[derive(PartialEq, Properties)]
pub struct EditorProps {
    pub article: Value,
}

fn markdown_to_html(markdown: &str) -> String {
    markdown::to_html_with_options(markdown, &markdown::Options::gfm()).unwrap()
}

#[function_component]
pub fn Editor(props: &EditorProps) -> Html {
    let navigator = use_navigator().unwrap();
    // let article_id = props.article_id.clone();
    let article = props.article.clone();
    let init_content = article["content"].as_str().unwrap();
    let init_subject = article["subject"].as_str().unwrap();
    let init_summary = article["summary"].as_str().unwrap();
    let init_category_id = article["category"]["id"].as_str().unwrap().to_string();
    let init_published = article["published"].as_bool().unwrap();
    let init_recommended = article["recommended"].as_bool().unwrap();
    let init_top = article["top"].as_bool().unwrap();
    let user_id = article["user"]["id"].as_str().unwrap().to_string();
    let article_id = article["id"].as_str().unwrap().to_string();
    // web_sys::console::log_1(&JsValue::from_str(&init_category_id));

    // let content = use_state(|| "".to_string());
    let markdown = use_state(|| init_content.to_string());
    let html = use_state(|| markdown_to_html(init_content));
    let subject = use_state(|| init_subject.to_string());
    let summary = use_state(|| init_summary.to_string());
    let categories: UseStateHandle<Vec<Value>> = use_state(Vec::new);
    // let categories = use_state(|| Vec::<Value>::new());
    let category_id = use_state(|| init_category_id.clone());
    let published = use_state(|| init_published);
    let recommended = use_state(|| init_recommended);
    let top = use_state(|| init_top);

    let published_text = {
        let published = published.clone();
        if *published {
            "Update"
        } else {
            "Publish"
        }
    };

    let preview_ref = use_node_ref();
    let editor_ref = use_node_ref();

    fn sync_scroll_position(source: &HtmlElement, target: &HtmlElement) {
        let source_scroll_top = source.scroll_top();
        let source_scroll_height = source.scroll_height() - source.client_height();
        let scroll_percentage = source_scroll_top as f64 / source_scroll_height as f64;

        let target_scroll_height = target.scroll_height() - target.client_height();
        target.set_scroll_top((target_scroll_height as f64 * scroll_percentage) as i32);
    }

    let sync_preview_scroll = {
        let preview_ref = preview_ref.clone();
        let editor_ref = editor_ref.clone();
        Callback::from(move |_: Event| {
            if let (Some(editor_element), Some(preview_element)) = (
                editor_ref.cast::<HtmlElement>(),
                preview_ref.cast::<HtmlElement>(),
            ) {
                sync_scroll_position(&editor_element, &preview_element);
            }
        })
    };

    // 监听预览滚动，更新编辑器滚动位置
    // let sync_editor_scroll = {
    //     let preview_ref = preview_ref.clone();
    //     let editor_ref = editor_ref.clone();
    //     Callback::from(move |_: Event| {
    //         if let (Some(editor_element), Some(preview_element)) =
    //             (editor_ref.cast::<HtmlElement>(), preview_ref.cast::<HtmlElement>()) {
    //             sync_scroll_position(&preview_element, &editor_element);
    //         }
    //     })
    // };

    use_effect_with(markdown.clone(), {
        let markdown = markdown.clone();
        let html = html.clone();
        // let sync_heights = sync_heights.clone();
        let categories = categories.clone();
        move |_| {
            html.set(markdown_to_html(&markdown));
            spawn_local(async move {
                let data = fetch_categories_list()
                    .await
                    .unwrap()
                    .get("categories")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .to_vec();
                categories.set(data.clone());
                // categories.set(data.to_vec().clone());
            });
            || ()
        }
    });

    let oninput = {
        let markdown = markdown.clone();
        let html = html.clone();

        Callback::from(move |e: InputEvent| {
            e.prevent_default();
            let input_text = e.target_dyn_into::<HtmlTextAreaElement>().unwrap().value();
            markdown.set(input_text);
            html.set(markdown::to_html(&markdown));
        })
    };

    let check_top_click = {
        let top = top.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();
            top.set(!(*top));
        })
    };

    let check_recommended_click = {
        let recommended = recommended.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();
            recommended.set(!(*recommended));
        })
    };

    let on_subject_change = {
        let subject = subject.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            subject.set(input.value());
            web_sys::console::log_1(&JsValue::from_str(&subject.clone()));
        })
    };

    let on_category_change = {
        let category_id = category_id.clone();
        Callback::from(move |e: Event| {
            let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            // let value = select.value();
            category_id.set(select.value());
            web_sys::console::log_1(&JsValue::from_str(&select.value()));
        })
    };

    let on_summary_input = {
        let summary = summary.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            summary.set(input.value());
            web_sys::console::log_1(&JsValue::from_str(&input.value()));
        })
    };

    let publish_article = {
        let navigator = navigator.clone();
        let markdown = markdown.clone();
        let subject = subject.clone();
        let summary = summary.clone();
        let category_id = category_id.clone();
        let content = markdown.clone();
        let recommended = recommended.clone();
        let top = top.clone();
        let article_id = article_id.clone();
        let user_id = user_id.clone();
        Callback::from(move |_| {
            let navigator = navigator.clone();
            let subject = (*subject).clone();
            let summary = summary.clone();
            let category_id = category_id.clone();
            let content = content.clone();
            let recommended = recommended.clone();
            let top = top.clone();
            let article_id = article_id.clone();
            let user_id = user_id.clone();
            spawn_local(async move {
                let token = get_pair_value("jwt").unwrap();
                let new_flag = article_id.is_empty();
                let data = match new_flag {
                    true => new_article_data(
                        user_id.to_string().clone(),
                        subject.clone(),
                        (*category_id).clone(),
                        (*summary).clone(),
                        (*content).clone(),
                        true,
                        *top.clone(),
                        *recommended.clone(),
                    )
                    .await
                    .unwrap(),
                    false => update_article_data(
                        article_id.to_string(),
                        user_id.to_string().clone(),
                        subject.clone(),
                        (*category_id).clone(),
                        (*summary).clone(),
                        (*content).clone(),
                        true,
                        *top.clone(),
                        *recommended.clone(),
                        token,
                    )
                    .await
                    .unwrap(),
                };
                web_sys::console::log_1(&JsValue::from_str(&data.to_string()));
            });
            // navigator.push(&ManageRoute::Profile);
            navigator.back();
        })
    };

    let save_as_craft = {
        let navigator = navigator.clone();
        let markdown = markdown.clone();
        let subject = subject.clone();
        let summary = summary.clone();
        let category_id = category_id.clone();
        let content = markdown.clone();
        let recommended = recommended.clone();
        let top = top.clone();
        let article_id = article_id.clone();
        let user_id = user_id.clone();
        Callback::from(move |_| {
            let navigator = navigator.clone();
            let subject = (*subject).clone();
            let summary = summary.clone();
            let category_id = category_id.clone();
            let content = content.clone();
            let recommended = recommended.clone();
            let top = top.clone();
            let article_id = article_id.clone();
            let user_id = user_id.clone();
            spawn_local(async move {
                let token = get_pair_value("jwt").unwrap();
                let new_flag = article_id.is_empty();
                let data = match new_flag {
                    true => new_article_data(
                        user_id.to_string().clone(),
                        subject.clone(),
                        (*category_id).clone(),
                        (*summary).clone(),
                        (*content).clone(),
                        false,
                        *top.clone(),
                        *recommended.clone(),
                    )
                    .await
                    .unwrap(),
                    false => update_article_data(
                        article_id.to_string(),
                        user_id.to_string().clone(),
                        subject.clone(),
                        (*category_id).clone(),
                        (*summary).clone(),
                        (*content).clone(),
                        false,
                        *top.clone(),
                        *recommended.clone(),
                        token,
                    )
                    .await
                    .unwrap(),
                };
                web_sys::console::log_1(&JsValue::from_str(&data.to_string()));
            });
            // navigator.push(&ManageRoute::Profile);
            navigator.back();
        })
    };

    let delete_article = {
        let navigator = navigator.clone();
        let article_id = article_id.clone();
        Callback::from(move |_| {
            let article_id = article_id.clone();
            spawn_local(async move {
                let token = get_pair_value("jwt").unwrap();
                let data = delete_article_data(article_id.to_string().clone(), token)
                    .await
                    .unwrap();
                web_sys::console::log_1(&JsValue::from_str(&data.to_string()));
            });
            navigator.push(&ManageRoute::Profile);
            // web_sys::console::log_1(&JsValue::from_str(&article_id.clone()));
        })
    };

    html! {
        <div class="container h-100">
            <div class="row mb-4 my-3">
                <div class="col-md-12">
                    <div class="row mb-2">
                        <div class="col-md-6 mb-2">
                            <input type="text" oninput={on_subject_change} class="form-control" spellcheck="false" placeholder="Title" style="font-family: monospace" value={(*subject).clone()} />
                        </div>
                        <div class="col-md-6">
                            <div class="row">
                                <div class="col">
                                    <select class="form-select"
                                        onchange={on_category_change}
                                    >
                                        {
                                            (*categories).clone().into_iter().map(|category| {
                                                let id = category["id"].as_str().unwrap().to_string();
                                                if (*category_id).clone() == id.clone() {
                                                    html! {
                                                        <option selected=true value={id.clone()}>{category["name"].as_str().unwrap()}</option>
                                                    }
                                                } else {
                                                    html! {
                                                        <option value={id.clone()}>{category["name"].as_str().unwrap()}</option>
                                                    }
                                                }
                                            }).collect::<Html>()
                                        }
                                    </select>
                                </div>
                                <div class="col">
                                    // <input type="button" class="form-control" value="Topics" />
                                    <div class="btn-group" role="group" aria-label="Basic checkbox toggle button group">
                                      <input onchange={check_top_click} type="checkbox" class="btn-check" id="chk_top" autocomplete="off" checked={*top}/>
                                      <label class="btn btn-outline-primary" for="chk_top">{"top"}</label>

                                      <input onchange={check_recommended_click} type="checkbox" class="btn-check" id="chk_recommended" autocomplete="off" checked={*recommended} />
                                      <label class="btn btn-outline-primary" for="chk_recommended">{"recommended"}</label>
                                    </div>
                                </div>
                                <div class="col">
                                    <div class="btn-group">
                                      <button onclick={publish_article} type="button" class="btn btn-primary">
                                            {published_text}
                                      </button>
                                      <button type="button" class="btn btn-primary dropdown-toggle dropdown-toggle-split" data-bs-toggle="dropdown" aria-expanded="false">
                                        <span class="visually-hidden">{"Toggle Dropdown"}</span>
                                      </button>
                                      <ul class="dropdown-menu">
                                        <li><button onclick={
                                            save_as_craft
                                        } class="dropdown-item" >{"Save As Craft"}</button></li>
                                        <li><hr class="dropdown-divider" /></li>
                                        <li><button onclick={
                                            delete_article
                                        } class="dropdown-item btn btn-danger" >{"Delete"}</button></li>
                                      </ul>
                                    </div>
                                    // <button onclick={publish_article} type="button" class="btn btn-primary form-control">{{"publish"}}</button>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="row mb-2">
                        <div class="col-md-12">
                            <textarea oninput={on_summary_input} class="form-control h-100" spellcheck="false" style="font-family: monospace; overflow: auto;" placeholder="Summary" value={(*summary).clone()} ></textarea>
                        </div>
                    </div>
                </div>
                // <div class="row">
                //     <div class="col-md-12 form-floating">
                //         <textarea class="form-control h-100" placeholder="Summary" id="ta_summary" rows="4"></textarea>
                //         <label for="ta_summary">{"Comments"}</label>
                //     </div>
                // </div>
                // <div class="container mt-4">
                //   <div>
                //     <div>
                //     //   <!-- Form structure -->
                //       <form>
                //         // <!-- Row for Title, Category, Topics -->
                //         <div class="row mb-3">
                //           <div class="col-md-4">
                //             <input type="text" class="form-control" placeholder="Title" />
                //           </div>
                //           <div class="col-md-3">
                //             <select class="form-select">
                //             //   <option selected>{"Category"}</option>
                //               <option value="1">{"Category 1"}</option>
                //               <option selected=true value="2">{"Category 2"}</option>
                //               <option value="3">{"Category 3"}</option>
                //             </select>
                //           </div>
                //           <div class="col-md-3">
                //             <input type="button" class="form-control" value="Topics" />
                //           </div>
                //           <div class="col-md-2 d-flex flex-column justify-content-between">
                //             <button type="button" class="btn btn-primary mb-2">{"Category"}</button>
                //             // <!-- <button type="button" class="btn btn-secondary">Craft</button> -->
                //           </div>
                //         </div>
                //         // <!-- Row for Summary -->
                //         <div class="row mb-3">
                //           <div class="col-12 form-floating ">
                //             <textarea class="form-control" placeholder="Summary" id="ta_summary" rows="3"></textarea>
                //             <label for="ta_summary">{"Comments"}</label>
                //           </div>
                //         </div>
                //       </form>
                //     </div>
                //   </div>
                // </div>
            </div>

            <div class="row h-75">
                // <div class="col-md-6 h-100">
                //     <textarea
                //         class="form-control h-100"
                //         style="overflow: auto;"
                //         {oninput}
                //         onscroll={sync_preview_scroll.clone()}
                //         ref={editor_ref}
                //         value={(*markdown).clone()}
                //     />
                // </div>
                // <div class="col-md-6 h-100" style="overflow: auto;"
                //     onscroll={sync_editor_scroll.clone()}
                //     ref={preview_ref}
                // >
                //     {
                //         {
                //             let content_html_section = gloo_utils::document().create_element("section").unwrap();
                //             content_html_section.set_inner_html((*html).as_str());
                //             let content_html_node = Html::VRef(content_html_section.into());
                //             content_html_node
                //         }
                //     }
                // </div>
                <div class="article-editor-editor col-md-6 h-100 mb-4">
                    <div class="bordered h-100">
                        // <h5 class="text-center">{"Editor"}</h5>
                        <textarea
                            class="form-control h-100"
                            style="overflow: auto; font-family: monospace;"
                            spellcheck="false"
                            {oninput}
                            onscroll={sync_preview_scroll.clone()}
                            ref={editor_ref}
                            value={(*markdown).clone()}
                        ></textarea>
                    </div>
                </div>
                <div class="article-editor-preview col-md-6 h-100">
                    <div class="bordered h-100">
                        // <h5 class="text-center">{"Preview"}</h5>
                        <div class="p-2 markdown-body h-100"
                        style="overflow: auto;"
                        // onscroll={sync_editor_scroll.clone()}
                        ref={preview_ref}>
                            <h1>{(*subject).clone()}</h1>
                            {
                                {
                                    let content_html_section = gloo_utils::document().create_element("section").unwrap();
                                    content_html_section.set_inner_html((*html).as_str());
                                    Html::VRef(content_html_section.into())
                                }
                            }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

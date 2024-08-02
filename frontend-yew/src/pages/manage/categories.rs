use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};

use crate::services::categories::create_category_by_token;
use crate::services::categories::delete_category_data;
use crate::services::categories::fetch_categories_list;
use crate::services::categories::update_category_data;

use crate::components::top_container::manage_top_container::ManageTopContainer;
use crate::utils::common::format_date;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Category {
    id: String,
    name: String,
    description: String,
    slug: String,
    articles: Vec<Value>,
    updated_at: String,
    created_at: String,
}

#[derive(PartialEq, Properties)]
pub struct CategoryTableProps {
    categories: Vec<Category>,
    on_delete: Callback<String>,
    on_update: Callback<(String, String, String)>,
}

#[function_component]
pub fn CategoryTable(props: &CategoryTableProps) -> Html {
    let categories = props.categories.clone();
    let editing = use_state(|| None::<String>); // Track which category is being edited
    let name = use_state(|| String::new());
    let description = use_state(|| String::new());

    html! {
        <table class="table">
          <thead>
            <tr>
            //   <th scope="col">#</th>
              <th scope="col">{"Name"}</th>
              <th scope="col">{"Description"}</th>
              <th scope="col">{"Articles"}</th>
              <th scope="col">{"Created at"}</th>
              <th scope="col">{"Updated at"}</th>
              <th scope="col">{"Operations"}</th>
            </tr>
          </thead>
          <tbody>
            {
                for categories.into_iter().map(|category| {
                    let id = category.id.clone();


                    let is_editing = *editing == Some(id.clone());

                    let on_edit_click = {
                        let category = category.clone();
                        let editing = editing.clone();
                        let id = id.clone();
                        let name = name.clone();
                        let description = description.clone();
                        Callback::from(move |_| {
                            editing.set(Some(id.clone()));
                            name.set(category.name.clone());
                            description.set(category.description.clone());
                        })
                    };

                    let on_save_click = {
                        let editing = editing.clone();
                        let id = id.clone();
                        let on_update = props.on_update.clone();
                        let name = name.clone();
                        let description = description.clone();
                        Callback::from(move |_| {
                            on_update.emit((id.clone(), (*name).clone(), (*description).clone()));
                            editing.set(None);
                        })
                    };

                    let on_name_change = {
                        let name = name.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            name.set(input.value());
                        })
                    };

                    let on_description_change = {
                        let description = description.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            description.set(input.value());
                        })
                    };

                    html! {
                          <tr key={id.clone()}>
                            <td>
                                { if is_editing {
                                    html! {
                                        <input
                                            type="text"
                                            value={(*name).clone()}
                                            oninput={on_name_change.clone()}
                                        />
                                    }
                                } else {
                                    html! { &category.name }
                                }}
                            </td>
                            <td>
                                { if is_editing {
                                    html! {
                                        <input
                                            type="text"
                                            value={(*description).clone()}
                                            oninput={on_description_change.clone()}
                                        />
                                    }
                                } else {
                                    html! { &category.description }
                                }}
                            </td>
                            <td>
                                {format!{"{}", category.articles.len()}}
                            </td>
                            <td>
                                {format!{"{}", format_date(&Value::String(category.created_at.clone()), "%m/%d/%Y %H:%M").unwrap()}}
                            </td>
                            <td>
                                {format!{"{}", format_date(&Value::String(category.updated_at.clone()), "%m/%d/%Y %H:%M").unwrap()}}
                            </td>
                            <td>
                                { if is_editing {
                                    html! {
                                        <button onclick={on_save_click}>{ "Save" }</button>
                                    }
                                } else {
                                    html! {
                                        <>
                                        <div class="btn-group" role="group" aria-label="Basic outlined example">
                                            <button type="button" class="btn btn-outline-primary" onclick={on_edit_click}>{ "Edit" }</button>
                                            <button type="button" class="btn btn-outline-primary" onclick={props.on_delete.reform(move |_| id.clone())}>{ "Delete" }</button>
                                            // <button type="button" class="btn btn-outline-primary">Right</button>
                                        </div>
                                            // <button onclick={on_edit_click}>{ "Edit" }</button>
                                            // <button onclick={props.on_delete.reform(move |_| id.clone())}>{ "Delete" }</button>
                                            // <button onclick={props.on_update.reform(move |_| (category.id.clone(), category.name.clone(), category.description.clone()))}>{ "Update" }</button>
                                        </>
                                    }
                                }}
                            </td>
                        </tr>
                    }
                })
            }
          </tbody>
        </table>
    }
}

#[derive(Properties, PartialEq)]
struct CategoryFormProps {
    on_add: Callback<(String, String)>,
    on_update: Callback<(String, String, String)>,
}

#[function_component]
fn CategoryForm(props: &CategoryFormProps) -> Html {
    let name_ref = use_node_ref();
    let desc_ref = use_node_ref();

    let onsubmit = {
        let name_ref = name_ref.clone();
        let desc_ref = desc_ref.clone();
        let on_add = props.on_add.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let name = name_ref.cast::<HtmlInputElement>().unwrap();
            let description = desc_ref.cast::<HtmlInputElement>().unwrap();
            on_add.emit((name.value(), description.value()));
            name.set_value("");
            description.set_value("");
        })
    };

    html! {
        <form onsubmit={onsubmit} class="row">
            <div class="mb-3 col">
                <label for="input_name" class="form-label">{"Name"}</label>
                <input ref={name_ref} type="text" class="form-control" id="input_name" required=true/>
            </div>
            <div class="mb-3 col">
                <label for="input_desc" class="form-label">{"Description"}</label>
                <input ref={desc_ref} type="text" class="form-control" id="input_desc" required=true/>
            </div>
            // <input ref={name_ref} type="text" placeholder="Name" />
            // <input ref={desc_ref} type="text" placeholder="Description" />
            <div class="col d-flex justify-content-start align-items-center" >
                <button type="submit" class="btn btn-primary">{ "Add Category" }</button>
            </div>
        </form>
    }
}

#[derive(PartialEq, Properties)]
pub struct CategoryManagePageProps {}

#[function_component]
pub fn CategoryManagePage(props: &CategoryManagePageProps) -> Html {
    let CategoryManagePageProps {} = props;
    let categories = use_state(|| Vec::new());
    let error = use_state(|| None);

    {
        let categories = categories.clone();
        use_effect_with((), move |_| {
            let categories = categories.clone();
            spawn_local(async move {
                match fetch_categories_list().await {
                    Ok(data) => {
                        let data = data["categories"].as_array().unwrap().to_vec();
                        categories.set(
                            data.into_iter()
                                .map(|c| Category {
                                    id: c["id"].as_str().unwrap().to_string(),
                                    name: c["name"].as_str().unwrap().to_string(),
                                    description: c["description"].as_str().unwrap().to_string(),
                                    slug: c["slug"].as_str().unwrap().to_string(),
                                    articles: c["articles"].as_array().unwrap().to_vec(),
                                    updated_at: c["updatedAt"].as_str().unwrap().to_string(),
                                    created_at: c["createdAt"].as_str().unwrap().to_string(),
                                })
                                .collect(),
                        );
                    }
                    Err(e) => {
                        web_sys::console::log_1(&JsValue::from_str(e.to_string().as_str()));
                    }
                }
            });
            || ()
        });
    }

    let add_category = {
        let categories = categories.clone();
        let error = error.clone();
        Callback::from(move |(name, description): (String, String)| {
            let categories = categories.clone();
            let error = error.clone();
            spawn_local(async move {
                match create_category_by_token(name.clone(), description.clone()).await {
                    Ok(data) => {
                        let data = data.get("categoryNewByToken").unwrap();
                        let mut updated_categories = (*categories).clone();
                        updated_categories.push(Category {
                            id: data["id"].as_str().unwrap().to_string(),
                            name: data["name"].as_str().unwrap().to_string(),
                            description: data["description"].as_str().unwrap().to_string(),
                            slug: data["slug"].as_str().unwrap().to_string(),
                            articles: vec![],
                            updated_at: data["updatedAt"].as_str().unwrap().to_string(),
                            created_at: data["createdAt"].as_str().unwrap().to_string(),
                        });
                        categories.set(updated_categories);
                        error.set(None);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&JsValue::from_str(e.to_string().as_str()));
                        error.set(Some(e.to_string()));
                    }
                }
            });
        })
    };

    let update_category = {
        let categories = categories.clone();
        let error = error.clone();
        Callback::from(move |(id, name, description): (String, String, String)| {
            let categories = categories.clone();
            let error = error.clone();
            spawn_local(async move {
                match update_category_data(id.clone(), name.clone(), description.clone()).await {
                    Ok(data) => {
                        let data = data.get("categoryUpdate").unwrap();
                        let mut updated_categories = (*categories).clone();
                        if let Some(category) =
                            updated_categories.iter_mut().find(|c| c.id == id.clone())
                        {
                            category.name = name.clone();
                            category.description = description.clone();
                            category.updated_at = data["updatedAt"].as_str().unwrap().to_string();
                        }
                        categories.set(updated_categories);
                        error.set(None);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&JsValue::from_str(e.to_string().as_str()));
                        error.set(Some(e.to_string()));
                    }
                }
                // let task = FetchService::fetch(request, callback).expect("Failed to start request");
                // task.await;
            });
        })
    };

    let delete_category = {
        let categories = categories.clone();
        let error = error.clone();
        Callback::from(move |id: String| {
            let categories = categories.clone();
            let error = error.clone();
            spawn_local(async move {
                match delete_category_data(id.clone()).await {
                    Ok(_) => {
                        let updated_categories = (*categories)
                            .clone()
                            .into_iter()
                            .filter(|c| c.id != id.clone())
                            .collect();
                        categories.set(updated_categories);
                        error.set(None);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&JsValue::from_str(e.to_string().as_str()));
                        error.set(Some(e.clone().to_string()));
                    }
                }
            });
        })
    };
    html! {
        <>
        <ManageTopContainer />
        <main class="container">
            <h1>{ "Category Manager" }</h1>
            <CategoryTable categories={(*categories).clone()} on_delete={delete_category.clone()} on_update={update_category.clone()}/>
            <CategoryForm on_add={add_category.clone()} on_update={update_category.clone()} />
            { if let Some(error) = &*error {
                // html! { <p>{ error }</p> }
                html! {
                <div class="alert alert-primary" role="alert">
                    { error }
                </div>
                }
            } else {
                html! {}
            }}
        </main>
        </>

    }
}

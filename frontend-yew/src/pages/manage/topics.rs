use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};

use crate::services::topics::create_topic_by_token;
use crate::services::topics::delete_topic_data;
use crate::services::topics::fetch_topics_list;
use crate::services::topics::update_topic_data;

use crate::components::top_container::manage_top_container::ManageTopContainer;
use crate::utils::common::format_date;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Topic {
    id: String,
    name: String,
    slug: String,
    articles: Vec<Value>,
    updated_at: String,
    created_at: String,
}

#[derive(PartialEq, Properties)]
pub struct TopicTableProps {
    topics: Vec<Topic>,
    on_delete: Callback<String>,
    on_update: Callback<(String, String)>,
}

#[function_component]
pub fn TopicTable(props: &TopicTableProps) -> Html {
    let topics = props.topics.clone();
    let editing = use_state(|| None::<String>); // Track which topic is being edited
    let name = use_state(String::new);

    html! {
        <table class="table">
          <thead>
            <tr>
            //   <th scope="col">#</th>
              <th scope="col">{"Name"}</th>
              <th scope="col">{"Articles"}</th>
              <th scope="col">{"Created at"}</th>
              <th scope="col">{"Updated at"}</th>
              <th scope="col">{"Operations"}</th>
            </tr>
          </thead>
          <tbody>
            {
                for topics.into_iter().map(|topic| {
                    let id = topic.id.clone();


                    let is_editing = *editing == Some(id.clone());

                    let on_edit_click = {
                        let topic = topic.clone();
                        let editing = editing.clone();
                        let id = id.clone();
                        let name = name.clone();
                        Callback::from(move |_| {
                            editing.set(Some(id.clone()));
                            name.set(topic.name.clone());
                        })
                    };

                    let on_save_click = {
                        let editing = editing.clone();
                        let id = id.clone();
                        let on_update = props.on_update.clone();
                        let name = name.clone();
                        Callback::from(move |_| {
                            on_update.emit((id.clone(), (*name).clone()));
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
                                    html! { &topic.name }
                                }}
                            </td>

                            <td>
                                {format!{"{}", topic.articles.len()}}
                            </td>
                            <td>
                                {format!{"{}", format_date(&Value::String(topic.created_at.clone()), "%m/%d/%Y %H:%M").unwrap()}}
                            </td>
                            <td>
                                {format!{"{}", format_date(&Value::String(topic.updated_at.clone()), "%m/%d/%Y %H:%M").unwrap()}}
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
                                            // <button onclick={props.on_update.reform(move |_| (topic.id.clone(), topic.name.clone(), topic.description.clone()))}>{ "Update" }</button>
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
struct TopicFormProps {
    on_add: Callback<String>,
    on_update: Callback<(String, String)>,
}

#[function_component]
fn TopicForm(props: &TopicFormProps) -> Html {
    let name_ref = use_node_ref();

    let onsubmit = {
        let name_ref = name_ref.clone();
        let on_add = props.on_add.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let name = name_ref.cast::<HtmlInputElement>().unwrap();
            on_add.emit(name.value());
            name.set_value("");
        })
    };

    html! {
        <form onsubmit={onsubmit} class="row">
            <div class="mb-3 col">
                <label for="input_name" class="form-label">{"Name"}</label>
                <input ref={name_ref} type="text" class="form-control" id="input_name" required=true/>
            </div>
            // <input ref={name_ref} type="text" placeholder="Name" />
            // <input ref={desc_ref} type="text" placeholder="Description" />
            <div class="col d-flex justify-content-start align-items-center" >
                <button type="submit" class="btn btn-primary">{ "Add Topic" }</button>
            </div>
        </form>
    }
}

#[derive(PartialEq, Properties)]
pub struct TopicManagePageProps {}

#[function_component]
pub fn TopicManagePage(props: &TopicManagePageProps) -> Html {
    let TopicManagePageProps {} = props;
    let topics = use_state(Vec::new);
    let error = use_state(|| None);

    {
        let topics = topics.clone();
        use_effect_with((), move |_| {
            let topics = topics.clone();
            spawn_local(async move {
                match fetch_topics_list().await {
                    Ok(data) => {
                        let data = data["topics"].as_array().unwrap().to_vec();
                        topics.set(
                            data.into_iter()
                                .map(|c| Topic {
                                    id: c["id"].as_str().unwrap().to_string(),
                                    name: c["name"].as_str().unwrap().to_string(),
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

    let add_topic = {
        let topics = topics.clone();
        let error = error.clone();
        Callback::from(move |name: String| {
            let topics = topics.clone();
            let error = error.clone();
            spawn_local(async move {
                match create_topic_by_token(name.clone()).await {
                    Ok(data) => {
                        let data = data.get("topicNewByToken").unwrap();
                        let mut updated_topics = (*topics).clone();
                        updated_topics.push(Topic {
                            id: data["id"].as_str().unwrap().to_string(),
                            name: data["name"].as_str().unwrap().to_string(),
                            slug: data["slug"].as_str().unwrap().to_string(),
                            articles: vec![],
                            updated_at: data["updatedAt"].as_str().unwrap().to_string(),
                            created_at: data["createdAt"].as_str().unwrap().to_string(),
                        });
                        topics.set(updated_topics);
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

    let update_topic = {
        let topics = topics.clone();
        let error = error.clone();
        Callback::from(move |(id, name): (String, String)| {
            let topics = topics.clone();
            let error = error.clone();
            spawn_local(async move {
                match update_topic_data(id.clone(), name.clone()).await {
                    Ok(data) => {
                        let data = data.get("topicUpdate").unwrap();
                        let mut updated_topics = (*topics).clone();
                        if let Some(topic) = updated_topics.iter_mut().find(|c| c.id == id.clone())
                        {
                            topic.name = name.clone();
                            topic.updated_at = data["updatedAt"].as_str().unwrap().to_string();
                        }
                        topics.set(updated_topics);
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

    let delete_topic = {
        let topics = topics.clone();
        let error = error.clone();
        Callback::from(move |id: String| {
            let topics = topics.clone();
            let error = error.clone();
            spawn_local(async move {
                match delete_topic_data(id.clone()).await {
                    Ok(_) => {
                        let updated_topics = (*topics)
                            .clone()
                            .into_iter()
                            .filter(|c| c.id != id.clone())
                            .collect();
                        topics.set(updated_topics);
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
            <h1>{ "Topic Manager" }</h1>
            <TopicTable topics={(*topics).clone()} on_delete={delete_topic.clone()} on_update={update_topic.clone()}/>
            <TopicForm on_add={add_topic.clone()} on_update={update_topic.clone()} />
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

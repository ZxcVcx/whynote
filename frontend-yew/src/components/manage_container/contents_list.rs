use crate::services::articles::{fetch_articles_data, fetch_crafts_data};
use crate::{app::ManageRoute, components::manage_container::content_card::ContentCard};
use gloo_utils::format::JsValueSerdeExt;
use serde_json::{json, Value};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlAnchorElement;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
use yew_router::prelude::*;

use std::collections::HashMap;
use std::io::Write;
use zip::write::SimpleFileOptions;

#[derive(PartialEq, Properties)]
pub struct ContentsListProps {
    pub crafts: Vec<Value>,
    pub articles: Vec<Value>,
    // pub navigator: Navigator<MainRoute>,
}

#[function_component]
pub fn ContentsList(props: &ContentsListProps) -> Html {
    let ContentsListProps { crafts, articles } = props;

    let export_json = {
        Callback::from(move |_| {
            spawn_local(async move {
                let crafts = fetch_crafts_data().await.unwrap();
                let articles = fetch_articles_data().await.unwrap();

                let data = json!({
                    "crafts": crafts,
                    "articles": articles,
                });

                let data = serde_json::to_string_pretty(&data).unwrap();
                let blob = web_sys::Blob::new_with_str_sequence_and_options(
                    &JsValue::from_serde(&[data]).unwrap(),
                    web_sys::BlobPropertyBag::new().type_("application/json"),
                )
                .unwrap();
                let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                let document = web_sys::window().unwrap().document().unwrap();
                let a = document
                    .create_element("a")
                    .unwrap()
                    .dyn_into::<HtmlAnchorElement>()
                    .unwrap();
                // let a: HtmlAnchorElement = document.create_element("a").unwrap().
                a.set_attribute("href", &url).unwrap();
                a.set_attribute("download", "data.json").unwrap();
                a.click();
                web_sys::Url::revoke_object_url(&url).unwrap();
            })
        })
    };
    let export_markdown = {
        Callback::from(move |_| {
            spawn_local(async move {
                let crafts = fetch_crafts_data()
                    .await
                    .unwrap()
                    .get("articles")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .to_vec();
                let articles = fetch_articles_data()
                    .await
                    .unwrap()
                    .get("articles")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .to_vec();
                let mut buffer = Vec::new();
                let mut zip_writer = zip::ZipWriter::new(std::io::Cursor::new(&mut buffer));

                // let options:FileOptions<ExtendedFileOptions> = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
                let options =
                    SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
                let add_files_to_zip =
                    |zip_writer: &mut zip::ZipWriter<std::io::Cursor<&mut Vec<u8>>>,
                     data: Vec<Value>,
                     folder_name: &str| {
                        let mut category_map = HashMap::new();
                        // let options = options.clone();
                        for item in data {
                            if let Some(category) = item["category"]["name"].as_str() {
                                category_map
                                    .entry(category.to_string())
                                    .or_insert(Vec::new())
                                    .push(item);
                                // category_map.entry(category.to_string()).or_default().push(item);
                            }
                        }

                        for (category, items) in category_map {
                            // zip_writer.start_file(format!("{}/{}.md", prefix, category), options).unwrap();
                            // for item in items {
                            //     let title = item.get("title").unwrap().as_str().unwrap();
                            //     let content = item.get("content").unwrap().as_str().unwrap();
                            //     zip_writer.write_all(format!("# {}\n\n{}", title, content).as_bytes()).unwrap();
                            // }
                            let folder_path = format!("{}/{}", folder_name, category);
                            for item in items {
                                let file_name = item["subject"].as_str().unwrap();
                                let file_path = format!("{}/{}.md", folder_path, file_name);
                                zip_writer.start_file(file_path, options).unwrap();
                                let content = item["content"].as_str().unwrap_or("");
                                zip_writer.write_all(content.as_bytes()).unwrap();
                            }
                        }
                    };

                add_files_to_zip(&mut zip_writer, crafts, "crafts");
                add_files_to_zip(&mut zip_writer, articles, "articles");

                zip_writer.finish().unwrap();

                let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
                    &js_sys::Array::of1(&js_sys::Uint8Array::from(buffer.as_slice())).into(),
                    web_sys::BlobPropertyBag::new().type_("application/zip"),
                )
                .unwrap();

                let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                let document = web_sys::window().unwrap().document().unwrap();
                let a = document
                    .create_element("a")
                    .unwrap()
                    .dyn_into::<HtmlAnchorElement>()
                    .unwrap();
                // let a: HtmlAnchorElement = document.create_element("a").unwrap().
                a.set_attribute("href", &url).unwrap();
                a.set_attribute("download", "data.zip").unwrap();
                a.click();
                web_sys::Url::revoke_object_url(&url).unwrap();
            })
        })
    };

    html! {
    <div class="p-2">
        <div class="row">
            <div class="col"></div>
            // <div class="col-4"></div>
            <div class="col d-flex justify-content-end align-items-center">
                <Link<ManageRoute> to={ManageRoute::NewEditor }>
                    <button class="btn btn-primary ms-2">{"Create"}</button>
                </Link<ManageRoute>>
                // <Link<ManageRoute> to={ManageRoute::NewEditor }>
                //     <button class="btn btn-secondary ms-2">{"Export"}</button>
                // </Link<ManageRoute>>
                // <button class="btn btn-secondary ms-2">{"Export"}</button>
                // <div class="btn-group">
                //     <button type="button" class="btn btn-secondary dropdown-toggle" data-bs-toggle="dropdown" aria-expanded="false">
                //         {"Import"}
                //     </button>
                //     <ul class="dropdown-menu">
                //         <li><a class="dropdown-item" href="#">{"Import from file"}</a></li>
                //         <li><a class="dropdown-item" href="#">{"Import from URL"}</a></li>
                //     </ul>
                // </div>
                <div class="btn-group ms-2">
                    <button type="button" class="btn btn-secondary dropdown-toggle" data-bs-toggle="dropdown" aria-expanded="false">
                        {"Export"}
                    </button>
                    <ul class="dropdown-menu">
                        <li><button class="dropdown-item" onclick={export_json}>{"Export as JSON"}</button></li>
                        <li><button class="dropdown-item" onclick={export_markdown}>{"Export as MarkDown"}</button></li>
                        // <li><a class="dropdown-item" href="#">{"Export as JSON"}</a></li>
                        // <li><a class="dropdown-item" href="#">{"Export as MarkDown"}</a></li>
                    </ul>
                </div>
                <Link<ManageRoute> to={ManageRoute::NewEditor }>
                    <button class="btn btn-danger ms-2">{"Delete All"}</button>
                </Link<ManageRoute>>
            </div>
        </div>
        <h3>{"crafts"}</h3>
        <ContentCard contents={crafts.clone()} published={false} />
        <h3 class="mt-5">{"articles"}</h3>
        <ContentCard contents={articles.clone()} published={true} />
    </div>
    }
}

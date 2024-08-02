use yew::{function_component, html, Html};

use std::collections::HashMap;
// use std::collections::HashSet;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{FileReader, HtmlInputElement, ProgressEvent};
use yew::events::Event;
use yew::prelude::*;
use zip::ZipArchive;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Category {
    name: String,
    description: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Article {
    title: String,
    content: String,
}

type FileMap = HashMap<Category, Vec<Article>>;

fn collect_next(zip: &mut ZipArchive<Cursor<Vec<u8>>>) -> FileMap {
    let mut files_map = FileMap::new();

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();

        if file.is_file() {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let name_buf = match file.enclosed_name() {
                Some(name) => name,
                None => continue,
            };
            let parent_dir_name = name_buf
                .parent()
                .and_then(Path::file_name)
                .map(|os_str| os_str.to_string_lossy().to_string())
                .unwrap_or_default();
            let file_name = name_buf
                .file_name()
                .map(|os_str| os_str.to_string_lossy().to_string())
                .unwrap_or_default();
            let category = Category {
                name: parent_dir_name.clone(),
                description: parent_dir_name.clone(),
            };
            let article = Article {
                title: file_name.strip_suffix(".md").unwrap().to_string().clone(),
                content: content.clone(),
            };
            files_map
                .entry(category)
                // .or_insert_with(Vec::new)
                .or_default()
                .push(article)
        }

        // if file.is_dir() {
        //     let name_buf = match file.enclosed_name() {
        //         Some(name) => name,
        //         None => continue,
        //     };
        //     let dir_name = name_buf
        //         .file_name()
        //         .map(|os_str| os_str.to_string_lossy().to_string())
        //         .unwrap_or_default();
        //     files_map.entry(dir_name).or_insert_with(Vec::new);
        // }
    }
    files_map
}

fn check_file_structure(zip: &mut ZipArchive<Cursor<Vec<u8>>>) -> bool {
    // let mut files_map = HashMap::new();

    for i in 0..zip.len() {
        let file = zip.by_index(i).unwrap();

        if file.is_file() {
            let name_buf = match file.enclosed_name() {
                Some(name) => name,
                None => continue,
            };
            // if file is not .md file, return false
            if let Some(extension) = name_buf.extension() {
                if extension != "md" {
                    return false;
                }
            } else {
                return false;
            }
            if name_buf.parent().and_then(Path::parent).is_some()
                && name_buf.parent().and_then(Path::parent).unwrap() == Path::new("")
            {
                return false;
            }
        }
    }
    true
}

#[function_component]
pub fn Test() -> Html {
    // let file_map = use_state(HashMap::<String, Vec<String>>::new);
    let file_map = use_state(|| FileMap::new());

    let on_file_change = {
        let file_map = file_map.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();

            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    // web_sys::console::log_1(&JsValue::from_str(file.name().clone().strip_suffix(".zip").unwrap()));

                    let reader = FileReader::new().unwrap();
                    let file_map = file_map.clone(); // 显式克隆状态
                    let onload = Closure::wrap(Box::new(move |e: ProgressEvent| {
                        let reader: FileReader = e.target().unwrap().dyn_into().unwrap();
                        let array_buffer = reader.result().unwrap();
                        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                        let bytes = uint8_array.to_vec();

                        let cursor = Cursor::new(bytes);
                        let mut zip = ZipArchive::new(cursor).unwrap();

                        let mut names = Vec::new();
                        for i in 0..zip.len() {
                            let file = zip.by_index(i).unwrap();
                            names.push(file.name().to_string());
                        }

                        if !check_file_structure(&mut zip) {
                            return;
                        }

                        // let files = collect_files(&mut zip);
                        let files = collect_next(&mut zip);

                        file_map.set(files);
                    }) as Box<dyn FnMut(_)>);

                    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    reader.read_as_array_buffer(&file).unwrap();
                    onload.forget();
                }
            }
        })
    };

    html! {
        <div>
            <ul>
                { for (*file_map).iter().map(|(category, articles)|
                     html!{ <li>{category.name.clone()} {category.description.clone()}
                        <ul>
                            { for (articles).iter().map(|article| html!{ <li>{article.title.clone()} {article.content.clone()} </li> }) }
                        </ul>
                    </li> })
                }
            </ul>
            <form>
                <input type="file" onchange={on_file_change} accept=".zip" required=true />
                <input type="submit" value="Submit"/>
            </form>
        </div>
    }
}

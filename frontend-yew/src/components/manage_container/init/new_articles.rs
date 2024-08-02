use crate::services::init::{fetch_new_articles_data, fetch_new_categories_data};
use crate::types::init::{ArticleNewType, ArticleType, CategoryNewType, UserNewToken};
use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, Html, Properties, UseStateSetter};

use std::collections::HashMap;
// use std::collections::HashSet;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{FileReader, HtmlInputElement, ProgressEvent};
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

fn collect(zip: &mut ZipArchive<Cursor<Vec<u8>>>) -> FileMap {
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

        if file.is_dir() {
            let name_buf = match file.enclosed_name() {
                Some(name) => name,
                None => continue,
            };
            let dir_name = name_buf
                .file_name()
                .map(|os_str| os_str.to_string_lossy().to_string())
                .unwrap_or_default();
            let category = Category {
                name: dir_name.clone(),
                description: dir_name.clone(),
            };
            // files_map.entry(category).or_insert_with(Vec::new);
            files_map.entry(category).or_default();
        }
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

#[derive(PartialEq, Properties)]
pub struct NewArticlesComponentProps {
    pub user_new_token: UserNewToken,
    pub articles_setter: UseStateSetter<Vec<ArticleType>>,
    pub categories_setter: UseStateSetter<Vec<CategoryNewType>>,
}

#[function_component]
pub fn NewArticlesComponent(props: &NewArticlesComponentProps) -> Html {
    // let NewArticlesComponentProps {} = props;
    let user_new_token = props.user_new_token.clone();
    let articles_setter = props.articles_setter.clone();
    let categories_setter = props.categories_setter.clone();

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
                        let files = collect(&mut zip);

                        file_map.set(files);
                    }) as Box<dyn FnMut(_)>);

                    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    reader.read_as_array_buffer(&file).unwrap();
                    onload.forget();
                }
            }
        })
    };

    let on_file_upload_submit = {
        let file_map = file_map.clone();
        let user_new_token = user_new_token.clone();
        let articles_setter = articles_setter.clone();
        let categories_setter = categories_setter.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let files = file_map.clone();
            let user_new_token = user_new_token.clone();
            let articles_setter = articles_setter.clone();
            let categories_setter = categories_setter.clone();

            let mut articles = vec![];
            let mut categories = vec![];

            spawn_local(async move {
                let mut cates = vec![];
                for (category, _) in files.iter() {
                    cates.push((category.name.as_str(), category.description.as_str()));
                }
                let cates_data = fetch_new_categories_data(cates, user_new_token.id.as_str()).await;
                match cates_data {
                    Ok(data) => {
                        for (category, _) in files.iter() {
                            let category_data = data[category.name.as_str()].clone();
                            let category_user_data = category_data["categoryUserNew"].clone();
                            let category = CategoryNewType {
                                id: category_user_data["categoryId"]
                                    .as_str()
                                    .unwrap()
                                    .to_string(),
                                name: category.name.clone(),
                                description: category.description.clone(),
                            };
                            categories.push(category);
                        }

                        let mut arts = vec![];
                        for (category, articles) in files.iter() {
                            for article in articles {
                                let art = ArticleNewType {
                                    user_id: user_new_token.id.clone(),
                                    category_id: categories
                                        .iter()
                                        .find(|cate| cate.name == category.name)
                                        .unwrap()
                                        .id
                                        .clone(),
                                    subject: article.title.clone(),
                                    content: article.content.clone(),
                                    summary: "Default Summary".to_string(),
                                    published: true,
                                    top: false,
                                    recommended: false,
                                };
                                arts.push(art);
                            }
                        }

                        let arts_data =
                            fetch_new_articles_data(user_new_token.id.as_str(), arts).await;
                        match arts_data {
                            Ok(data) => {
                                let articles_data = data["articlesNew"].clone();
                                for article_data in articles_data.as_array().unwrap().iter() {
                                    web_sys::console::log_1(
                                        &JsValue::from_serde(article_data).unwrap(),
                                    );
                                    let article = ArticleType {
                                        id: article_data["id"].as_str().unwrap().to_string(),
                                        user_id: article_data["userId"]
                                            .as_str()
                                            .unwrap()
                                            .to_string(),
                                        category_id: article_data["categoryId"]
                                            .as_str()
                                            .unwrap()
                                            .to_string(),
                                        subject: article_data["subject"]
                                            .as_str()
                                            .unwrap()
                                            .to_string(),
                                        summary: article_data["summary"]
                                            .as_str()
                                            .unwrap()
                                            .to_string(),
                                        slug: article_data["slug"].as_str().unwrap().to_string(),
                                        content: article_data["content"]
                                            .as_str()
                                            .unwrap()
                                            .to_string(),
                                        published: true,
                                        top: false,
                                        recommended: false,
                                    };
                                    // let article = ArticleNewType {
                                    //     user_id: user_new_token.id.clone(),
                                    //     category_id: article_data["id"]
                                    //         .as_str()
                                    //         .unwrap()
                                    //         .to_string(),
                                    //     subject: article_data["subject"].as_str().unwrap().to_string(),
                                    //     summary: article_data["summary"].as_str().unwrap().to_string(),
                                    //     content: article_data["content"].as_str().unwrap().to_string(),
                                    //     published: true,
                                    //     top: false,
                                    //     recommended: false,
                                    // };
                                    articles.push(article);
                                }
                            }
                            Err(e) => {
                                web_sys::console::log_1(&JsValue::from_str(e.to_string().as_str()));
                                // return;
                            }
                        }

                        categories_setter.set(categories);
                        articles_setter.set(articles);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&JsValue::from_str(e.to_string().as_str()));
                        // return;
                    }
                }
            });
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
            <form onsubmit={on_file_upload_submit}>
            // <form>
                <input type="file" onchange={on_file_change} accept=".zip" required=true />
                <input type="submit" value="Submit"/>
            </form>
        </div>
    }
}

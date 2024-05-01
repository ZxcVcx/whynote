// use gloo_utils::format::JsValueSerdeExt;
// use yew::{function_component, html, Html, Properties};

// use wasm_bindgen::prelude::*;


// #[wasm_bindgen(module = "/assets/easymde.min.js")]
// extern "C" {
//     pub type EasyMDE;

//     // #[wasm_bindgen(constructor)]
//     // pub fn new() -> EasyMDE;

//     #[wasm_bindgen(constructor)]
//     pub fn new() -> EasyMDE;

//     #[wasm_bindgen(method)]
//     pub fn value(this: &EasyMDE) -> String;
// }

// #[derive(PartialEq, Properties)]
// pub struct EditorProps {}

// #[function_component]
// pub fn Editor(props: &EditorProps) -> Html {
//     let EditorProps {} = props;
//     let markdown = "# Hi, *Pluto*! ";
//     let editor = EasyMDE::new();
//     html! {
//         <div>
//             <h1>{"Markdown Editor Example"}</h1>
//             <textarea id="editor" value={markdown}></textarea>
//         </div>
//     }
// }
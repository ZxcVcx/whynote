use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

// use gloo_utils::format::JsValueSerdeExt;
use serde_json::Value;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, RequestInit, RequestMode};

use crate::utils::constants::CFG;

pub async fn build_request(query: Value) -> Result<web_sys::Request, FetchError> {
    let api_url = gql_uri().await;
    let headers = Headers::new().unwrap();
    headers.set("Content-Type", "application/json").unwrap();
    headers.set("Accept", "application/json").unwrap();
    let mut req_opts = RequestInit::new();
    req_opts.headers(&headers);
    req_opts.method("POST");
    req_opts.body(Some(&JsValue::from_str(query.to_string().as_str())));
    req_opts.mode(RequestMode::Cors);

    let req = web_sys::Request::new_with_str_and_init(api_url.to_owned().as_str(), &req_opts)?;
    Ok(req)
}

pub async fn fetch_with_request(
    request: web_sys::Request,
) -> Result<web_sys::Response, FetchError> {
    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: web_sys::Response = resp_value.dyn_into().unwrap();
    Ok(resp)
}

pub async fn fetch_gql_data(query: Value) -> Result<serde_json::Value, FetchError> {
    let request = build_request(query).await?;
    let response = fetch_with_request(request).await?;
    let response_text = JsFuture::from(response.text()?).await?;
    let data_str = response_text.as_string().unwrap();
    let data_value: serde_json::Value = serde_json::from_str(&data_str).unwrap();
    Ok(data_value["data"].clone())
}

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}


pub async fn gql_uri() -> String {

    let addr = format!("{}://{}:{}", CFG.get("GQL_PROTOCOL").unwrap(), CFG.get("GQL_HOST").unwrap(), CFG.get("GQL_PORT").unwrap());
    let path = CFG.get("GQL_PATH").unwrap();

    format!("{}/{}", addr, path)
}

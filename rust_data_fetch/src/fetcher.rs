use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Request, RequestInit, Response};

pub struct Fetcher {
    url: String,
}

impl Fetcher {
    pub fn new(url: &str) -> Fetcher {
        Fetcher {
            url: url.to_string(),
        }
    }

    pub async fn use_query(&self, query: &str) -> Result<String, JsValue> {
        let url = format!("{}{}", self.url, query);
        let request = Request::new_with_str_and_init(&url, &RequestInit::new())?;

        let window = window().unwrap();
        let response = JsFuture::from(window.fetch_with_request(&request)).await?;
        let response: Response = response.dyn_into()?;

        // Convert this other `Promise` into a rust `Future`.
        let text: String = JsFuture::from(response.text()?).await?.as_string().unwrap();
        Ok(text)
    }
}

use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Request, RequestInit, Response};
pub enum FetchState {
    NotStarted,
    Fetching,
    Complete,
    Error(String),
}

pub struct Fetcher {
    url: String,
    cache: HashMap<String, Value>,
    state: FetchState, // 状態を追跡するための新しいフィールド
}

impl Fetcher {
    pub fn new(url: &str) -> Fetcher {
        Fetcher {
            url: url.to_string(),
            cache: HashMap::new(),
            state: FetchState::NotStarted,
        }
    }

    pub async fn use_query(&mut self, query: &str) -> Result<Value, JsValue> {
        // Check cache first
        if let Some(cached) = self.cache.get(query) {
            return Ok(cached.clone());
        }

        self.state = FetchState::Fetching;

        let url = format!("{}?{}", &self.url, query);

        let window = window().ok_or_else(|| JsValue::from_str("Could not get window object"))?;

        let request_init = RequestInit::new();
        let request = Request::new_with_str_and_init(&url, &request_init)
            .map_err(|_| JsValue::from_str("Could not create request"))?;

        let response_jsvalue = JsFuture::from(window.fetch_with_request(&request)).await?;
        let response: Response = response_jsvalue
            .dyn_into()
            .map_err(|_| JsValue::from_str("Could not cast to response"))?;

        let json_jsvalue = JsFuture::from(response.json()?).await?;

        if let Some(json) = json_jsvalue.as_f64() {
            // Check if response is not JSON string but a number
            let result: Value = json.into();
            self.cache.insert(query.to_string(), result.clone());
            self.state = FetchState::Complete;
            return Ok(result);
        } else {
            let json: Result<Value, _> = serde_json::from_str(
                &json_jsvalue
                    .as_string()
                    .ok_or(JsValue::from_str("Response was not a String"))?,
            )
            .map_err(|_| JsValue::from_str("Response was not JSON"));

            match json {
                Ok(value) => {
                    self.cache.insert(query.to_string(), value.clone());
                    self.state = FetchState::Complete;
                    Ok(value)
                }
                Err(e) => {
                    // Handle error here.
                    log::error!("Failed to get value from JSON");
                    Err(e)
                }
            }
        }
    }
}

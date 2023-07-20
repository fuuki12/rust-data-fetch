pub mod fetcher;

use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub async fn use_query(url: &str, query: &str) -> Result<JsValue, JsValue> {
    let mut fetcher = fetcher::Fetcher::new(url);
    match fetcher.use_query(query).await {
        Ok(value) => {
            let js_value = to_value(&value).unwrap_or(JsValue::NULL);
            Ok(js_value)
        }
        Err(e) => Err(e),
    }
}

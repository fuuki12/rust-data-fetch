pub mod fetcher;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub async fn use_query(url: &str, query: &str) -> Result<String, JsValue> {
    let fetcher = fetcher::Fetcher::new(url);
    fetcher.use_query(query).await
}

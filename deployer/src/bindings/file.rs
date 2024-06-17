
use crate::prelude::*;

use crate::Buffer;

#[wasm_bindgen(module = "fs-extra")]
extern "C" {
    #[wasm_bindgen(js_namespace = default, catch)]
    async fn readFile(path: &str, format: Option<&str>) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = default, catch)]
    async fn writeFile(path: &str, data: &JsValue, format: Option<&str>) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
pub async fn file_read_binary(path: &str) -> Result<Buffer, JsValue> {
    Ok(readFile(path, None).await?.unchecked_into())
}

#[wasm_bindgen]
pub async fn file_write_binary(path: &str, data: &JsValue) -> Result<(), JsValue> {
    writeFile(path, data, None).await?;
    Ok(())
}

#[wasm_bindgen]
pub async fn file_read_string(path: &str) -> Result<String, JsValue> {
    let value = readFile(path, Some("utf8")).await?;
    value.as_string().ok_or_else(|| JsValue::from_str("file content is not a string"))
}

#[wasm_bindgen]
pub async fn file_write_string(path: &str, data: &str) -> Result<(), JsValue> {
    writeFile(path, &JsValue::from_str(data), Some("utf8")).await?;
    Ok(())
}

#[wasm_bindgen(module = "path")]
extern "C" {
    // create wasm_bindgen bindings for the path module
    #[wasm_bindgen(js_name = resolve)]
    pub fn file_path(a: &str, b: &str) -> String;
}

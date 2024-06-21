// all the commands are available via the prelude
mod commands;
pub use commands::*;

use std::env;

use futures::future;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use anyhow::{Result, anyhow};
use wasm_bindgen::prelude::*;

async fn invoke<RESP: DeserializeOwned>(fn_name: &str, args: &impl Serialize) -> Result<RESP> {
    // Set via Taskfile env, just for easier debugging
    if option_env!("TAURI_INVOKE") == Some("mock") {
        future::pending().await
    } else {
        let args = serde_wasm_bindgen::to_value(args).map_err(|err| anyhow!("{:?}", err))?;
        match TAURI_INSTANCE.core().invoke(fn_name, &args).await {
            Ok(data) => serde_wasm_bindgen::from_value(data).map_err(|err| anyhow!("{:?}", err)),
            Err(err) => Err(anyhow!("{:?}", err)),
        }
    }
}

#[wasm_bindgen(js_namespace = ["window"])]
extern "C" {
    #[derive(Debug, Clone)]
    type TauriInstance;
    #[wasm_bindgen(js_name = "__TAURI__")]
    static TAURI_INSTANCE: TauriInstance;

    #[wasm_bindgen(getter, method)]
    fn core(this: &TauriInstance) -> TauriCore;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    type TauriCore;

    #[wasm_bindgen(catch, method)]
    async fn invoke(this: &TauriCore, fn_name: &str, args: &JsValue) -> Result<JsValue, JsValue>;
}
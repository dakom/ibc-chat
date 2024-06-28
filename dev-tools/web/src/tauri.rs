// all the commands are available via the prelude
mod commands;
pub use commands::*;
use js_sys::{Array, Uint8Array};
use web_sys::{Blob, BlobPropertyBag, Url};

use std::env;

use futures::future;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use anyhow::{anyhow, Context, Result};
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

async fn invoke_no_resp(fn_name: &str, args: &impl Serialize) -> Result<()> {
    // Set via Taskfile env, just for easier debugging
    if option_env!("TAURI_INVOKE") == Some("mock") {
        future::pending().await
    } else {
        let args = serde_wasm_bindgen::to_value(args).map_err(|err| anyhow!("{:?}", err))?;
        match TAURI_INSTANCE.core().invoke(fn_name, &args).await {
            Ok(data) => Ok(()), 
            Err(err) => Err(anyhow!("{:?}", err)),
        }
    }
}

async fn invoke_no_args_no_resp(fn_name: &str) -> Result<()> {
    // Set via Taskfile env, just for easier debugging
    if option_env!("TAURI_INVOKE") == Some("mock") {
        future::pending().await
    } else {
        match TAURI_INSTANCE.core().invoke(fn_name, &JsValue::null()).await {
            Ok(_) => Ok(()),
            Err(err) => Err(anyhow!("{:?}", err)),
        }
    }
}

async fn invoke_no_args<RESP: DeserializeOwned>(fn_name: &str) -> Result<RESP> {
    // Set via Taskfile env, just for easier debugging
    if option_env!("TAURI_INVOKE") == Some("mock") {
        future::pending().await
    } else {
        match TAURI_INSTANCE.core().invoke(fn_name, &JsValue::null()).await {
            Ok(data) => serde_wasm_bindgen::from_value(data).map_err(|err| anyhow!("{:?}", err)),
            Err(err) => Err(anyhow!("{:?}", err)),
        }
    }
}

#[derive(Debug, Deserialize)]
struct EventData<T> {
    event: String,
    payload: T,
}
async fn listen<F, T>(event_name: &str, mut callback: F) -> Result<Closure<dyn FnMut(wasm_bindgen::JsValue)>> 
where 
    F: FnMut(T) + 'static,
    T: DeserializeOwned,
{
    // Set via Taskfile env, just for easier debugging
    if option_env!("TAURI_INVOKE") == Some("mock") {
        future::pending().await
    } else {
        let callback = Closure::new(move |data:JsValue| {
            let data:Result<EventData<T>> = serde_wasm_bindgen::from_value(data).map_err(|err| anyhow!("{:?}", err));
            match data {
                Ok(data) => {
                    callback(data.payload);
                },
                Err(err) => log::error!("{:?}", err),
            }
        });

        TAURI_INSTANCE.event().listen(event_name, &callback).await.map_err(|err| anyhow!("{:?}", err))?;

        Ok(callback)
    }
}

async fn resolve_resource_path(path: &str) -> Result<String> {
    match TAURI_INSTANCE.path().resolve_resource(path).await {
        Ok(data) => data.as_string().context(format!("{} is not a string", path)),
        Err(err) => Err(anyhow!("{:?}", err)),
    }
}

async fn load_resource_bytes(path: &str) -> Result<Uint8Array> {
    let path = resolve_resource_path(path).await?;
    match TAURI_INSTANCE.fs().read_file(&path).await {
        Ok(data) => Ok(data.unchecked_into()), 
        Err(err) => Err(anyhow!("{:?}", err)),
    }
}

pub async fn resource_img_url(path: &str) -> Result<String> {
    let mime_type = mime_guess::from_path(path).first().context(format!("could not guess mime type from {}", path))?;
    let data = load_resource_bytes(path).await?;

    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(&mime_type.to_string());

    
    let blob = Blob::new_with_buffer_source_sequence_and_options(&Array::of1(&data).into(), &blob_opts).map_err(|err| anyhow!("{:?}", err))?;
    let url = Url::create_object_url_with_blob(&blob).map_err(|err| anyhow!("{:?}", err))?;

    Ok(url)
}

#[wasm_bindgen(js_namespace = ["window"])]
extern "C" {
    #[derive(Debug, Clone)]
    type TauriInstance;
    #[wasm_bindgen(js_name = "__TAURI__")]
    static TAURI_INSTANCE: TauriInstance;

    #[wasm_bindgen(getter, method)]
    fn core(this: &TauriInstance) -> TauriCoreApi;

    #[wasm_bindgen(getter, method)]
    fn event(this: &TauriInstance) -> TauriEventApi;

    #[wasm_bindgen(getter, method)]
    fn path(this: &TauriInstance) -> TauriPathApi;

    #[wasm_bindgen(getter, method)]
    fn fs(this: &TauriInstance) -> TauriFsApi;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    type TauriCoreApi;

    #[wasm_bindgen(catch, method)]
    async fn invoke(this: &TauriCoreApi, fn_name: &str, args: &JsValue) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    type TauriEventApi;

    #[wasm_bindgen(catch, method)]
    async fn listen(this: &TauriEventApi, event_name: &str, callback: &Closure<dyn FnMut(JsValue)>) -> Result<JsValue, JsValue>;
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    type TauriPathApi;

    #[wasm_bindgen(catch, method, js_name = "resolveResource")]
    async fn resolve_resource(this: &TauriPathApi, resource_path: &str) -> Result<JsValue, JsValue>;
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    type TauriFsApi;

    #[wasm_bindgen(catch, method, js_name = "readFile")]
    async fn read_file(this: &TauriFsApi, path: &str) -> Result<JsValue, JsValue>;
}

use core::panic;
use std::borrow::Cow;

use awsm_web::env::{self, env_var};
use cosmwasm_std::Addr;
use once_cell::sync::Lazy;
use web_sys::HtmlImageElement;

use crate::{prelude::*, route::Route};

#[derive(Debug)]
pub struct Config {
    // the part of the url that is not the domain
    // e.g. in http://example.com/foo/bar, this would be "foo" if we want
    // all parsing to start from /bar
    // it's helpful in shared hosting environments where the app is not at the root
    pub root_path: &'static str,
    pub default_lang: Option<&'static str>,
    // for debugging, jump into an initial page (will wait until wallet is connected, works with auto_connect)
    pub start_route: Mutex<Option<Route>>,
}

impl Config {
    pub fn media_root(&self) -> Cow<'_, str> {
        if option_env!("TAURI_MEDIA") == Some("localserver") {
            Cow::Borrowed("http://localhost:9001")
        } else {
            Cow::Owned("todo".to_string())
        }
    }

    pub async fn app_image_url(&self, path: &str) -> Result<String> {
        if option_env!("TAURI_MEDIA") == Some("localserver") {
            Ok(format!("http://localhost:9001/{}", path))
        } else {
            tauri::resource_img_url(&format!("tauri-media/{}", path)).await
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "dev")] {
        pub static CONFIG: Lazy<Config> = Lazy::new(|| {
            Config {
                root_path: "",
                default_lang: None,
                start_route: Mutex::new(None),
            }
        });
    } else {
        pub static CONFIG: Lazy<Config> = Lazy::new(|| {
            Config {
                root_path: "",
                default_lang: None,
                start_route: Mutex::new(None),
            }
        });
    }
}
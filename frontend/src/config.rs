use core::panic;

use awsm_web::env::{self, env_var};
use cosmwasm_std::Addr;
use once_cell::sync::Lazy;

use crate::{prelude::*, route::Route};
use wallet::config::Environment;

pub static CHAINENV: Lazy<Environment> = Lazy::new(|| option_env!("CHAINENV").unwrap_ext().into());

#[derive(Debug)]
pub struct Config {
    // the part of the url that is not the domain
    // e.g. in http://example.com/foo/bar, this would be "foo" if we want
    // all parsing to start from /bar
    // it's helpful in shared hosting environments where the app is not at the root
    pub root_path: &'static str,
    pub media_root: &'static str,
    pub default_lang: Option<&'static str>,
    // for debugging, auto connect to wallet
    pub auto_connect: bool,
    // for debugging, jump into an initial page (will wait until wallet is connected, works with auto_connect)
    pub start_route: Mutex<Option<Route>>,
    pub messages_poll_delay_ms: u32,
    pub events_poll_delay_ms: u32,
}

impl Config {
    pub fn app_image_url(&self, path: &str) -> String {
        format!("{}/{}", self.media_root, path)
    }
}



cfg_if::cfg_if! {
    if #[cfg(feature = "dev")] {
        pub static CONFIG: Lazy<Config> = Lazy::new(|| {
            Config {
                root_path: "",
                media_root: "http://localhost:9000",
                default_lang: None,
                auto_connect: true,
                //auto_connect: false,
                //start_route: Mutex::new(Some(Route::Chat)),
                start_route: Mutex::new(None),
                messages_poll_delay_ms: 1000,
                events_poll_delay_ms: 1000,
            }
        });
    } else {
        pub static CONFIG: Lazy<Config> = Lazy::new(|| {
            Config {
                root_path: "ibc-chat",
                media_root: "https://dakom.github.io/ibc-chat/media",
                default_lang: None,
                auto_connect: false,
                start_route: Mutex::new(None),
                messages_poll_delay_ms: 3000,
                events_poll_delay_ms: 3000,

            }
        });
    }
}
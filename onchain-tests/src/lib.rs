#![allow(warnings)]
mod prelude;
mod config;
mod args;
mod runner;
mod helpers;

use awsm_web::env::env_var;
use config::CONFIG;
use runner::run_all_tests;
use wallet::get_cosmjs_node;
use crate::prelude::*;


#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    init_logger();

    std::panic::set_hook(Box::new(on_panic));

    Wallet::connect(
        get_cosmjs_node(),
        env_var("CHAINENV").expect_ext("set CHAINENV env var").as_str().into(),
        Some(env_var("CLI_SEED_PHRASE").expect_ext("set CLI_SEED_PHRASE env var"))
    ).await.map_err(|err| JsValue::from_str(&err.to_string()))?;

    let res = run_all_tests().await;

    if let Err(err) = res {
        log::error!("{}", err.to_string());
    }

    Ok(())
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook"))] {
        fn init_logger() {
            wasm_logger::init(wasm_logger::Config::default());
            console_error_panic_hook::set_once();
            log::info!("rust logging enabled!!!");
        }
    } else {
        fn init_logger() {
            log::info!("rust logging disabled!"); //<-- won't be seen
        }
    }
}

fn on_panic(info: &std::panic::PanicInfo) {
    log::error!("panic: {:?}", info);
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::hook(info);
}
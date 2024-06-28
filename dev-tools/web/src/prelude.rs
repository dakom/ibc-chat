pub use anyhow::{Result, Context as AnyhowContext, bail, anyhow};
pub use awsm_web::prelude::*;
use dominator::DomBuilder;
pub use dominator::{
    clone, 
    events, 
    html, 
    svg, 
    with_node, 
    Dom,
    apply_methods,
    styles,
    Fragment,
    fragment,
    class,
    attrs,
    link,
};
pub use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
pub use serde::{Deserialize, Serialize};
pub use std::sync::{Arc, Mutex, RwLock};
pub use wasm_bindgen::prelude::*;

pub use shared::msg::contract::{
    client::{QueryMsg as ClientQueryMsg, ExecuteMsg as ClientExecuteMsg},
    server::QueryMsg as ServerQueryMsg,
};
pub use crate::{
    theme::{
        typography::*,
        color::*,
        misc::*,
    },
    get_text,
    tauri,
    page::settings::SETTINGS, 
};

pub use once_cell::sync::Lazy;

pub type MixinStub<T> = fn(DomBuilder<T>) -> DomBuilder<T>;
//pub type MixinFn<T, F> = F;

pub trait MixinFnOnce<T>: FnOnce(DomBuilder<T>) -> DomBuilder<T> {}
impl <T, F> MixinFnOnce<T> for F where F: FnOnce(DomBuilder<T>) -> DomBuilder<T> {}

pub trait MixinFn<T>: Fn(DomBuilder<T>) -> DomBuilder<T> {}
impl <T, F> MixinFn<T> for F where F: Fn(DomBuilder<T>) -> DomBuilder<T> {}
#![allow(warnings)]
pub mod config;
pub mod wallet;
pub mod response_types;
pub mod bindings;
pub mod contract_traits;
pub mod wallet_contract_impls;
pub mod prelude;

#[cfg(feature = "node")]
pub use crate::bindings::cosmjs::node::get_cosmjs as get_cosmjs_node;

#[cfg(feature = "web")]
pub use crate::bindings::cosmjs::web::get_cosmjs as get_cosmjs_web;
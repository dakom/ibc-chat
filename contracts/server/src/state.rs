pub mod ibc;

/// Generally speaking - all entry points get a State (read-only)
/// instantiate/execute/migrate get that _and_ a StateContext (writable)
use cosmwasm_std::{Api, Deps, DepsMut, Empty, Env, QuerierWrapper, Storage};
use anyhow::Result;
use cw2::get_contract_version;
use shared::{contract_kind::ContractKind, response::ResponseBuilder};


/// State is a wrapper around the environment and storage, and provides a simplified API
/// in larger applications, it may also do things like cache some values in memory, etc.
pub struct State<'a> {
    pub api: &'a dyn Api,
    pub env: Env,
    pub querier: QuerierWrapper<'a, Empty>,
}

/// StateContext is a wrapper around the mutable environment and storage, and provides a simplified API
/// for handing storage updates, events, etc.
pub struct StateContext<'a> {
    pub store: &'a mut dyn Storage,
    pub response: ResponseBuilder,
}

impl<'a> State<'a> {
    pub fn new(deps: Deps<'a>, env: Env) -> Result<(Self, &dyn Storage)> {
        Ok((
            State {
                api: deps.api,
                env,
                querier: deps.querier,
            },
            deps.storage,
        ))
    }
}

impl<'a> StateContext<'a> {
    pub fn new(deps: DepsMut<'a>, env: Env) -> Result<(State<'a>, Self)> {
        let contract_version = get_contract_version(deps.storage)?;
        Ok((
            State {
                api: deps.api,
                env,
                querier: deps.querier,
            },
            StateContext {
                store: deps.storage,
                response: ResponseBuilder::new(contract_version, ContractKind::Server),
            },
        ))
    }
}
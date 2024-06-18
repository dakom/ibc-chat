use cosmwasm_std::{Order, Storage};
use cw_storage_plus::{Bound, Item, Map};
use shared::msg::network::NetworkId;

use super::{State, StateContext};
use anyhow::Result;

const NETWORK_ID:Item<NetworkId> = Item::new("network-id");

impl State<'_> {
    pub fn get_network_id(&self, store: &dyn Storage) -> Result<NetworkId> {
        NETWORK_ID.load(store).map_err(|err| err.into())
    }

    pub fn set_network_id(&self, ctx: &mut StateContext, network_id: NetworkId) -> Result<()> {
        NETWORK_ID.save(ctx.store, &network_id).map_err(|err| err.into())
    }
}
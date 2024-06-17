use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, IbcChannel, Uint128};

use super::contract::client::ChatMessage;

#[cw_serde]
pub enum IbcExecuteMsg {
    SendMessageToServer {
        message: ChatMessage
    },
    SendMessageToClient {
        message: ChatMessage
    }
}

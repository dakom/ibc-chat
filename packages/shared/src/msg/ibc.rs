use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, IbcChannel, Uint128};

use super::chat_message::{ChatMessage, ChatMessageWithIndex};

#[cw_serde]
pub enum IbcExecuteMsg {
    SendMessageToServer {
        // includes the index for the sake of emitting events only
        // when the message is actually stored on each client, it will be assigned a new index
        message: ChatMessageWithIndex
    },
    SendMessageToClient {
        message: ChatMessage
    }
}

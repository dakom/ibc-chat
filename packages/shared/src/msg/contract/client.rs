use cosmwasm_schema::{QueryResponses, cw_serde};
use cosmwasm_std::{Addr, Binary, BlockInfo, IbcChannel, Uint64};
use cw_utils::Expiration;

use crate::msg::{chat_message::{ChatMessageIndex, ChatMessageWithIndex}, misc::Order, network::NetworkId};

#[cw_serde]
pub struct InstantiateMsg {
    pub network_id: NetworkId
}

#[cw_serde]
pub enum ExecuteMsg {
    SendMessage {
        message: String
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// * returns [ChatMessagesResp]
    #[returns(ChatMessagesResp)]
    ChatMessages {
        after_index: Option<ChatMessageIndex>,
        // default is [Order::Ascending]
        order: Option<Order>
    },

    /// Get general information about the contract 
    #[returns(InfoResp)]
    Info { }
}

#[cw_serde]
pub struct InfoResp {
    pub server_channel: Option<IbcChannel>,
    pub network_id: NetworkId,
}

/// Placeholder migration message
#[cw_serde]
pub struct MigrateMsg {}

/// Response for [QueryMsg::ChatMessages]
#[cw_serde]
pub struct ChatMessagesResp {
    pub messages: Vec<ChatMessageWithIndex>,
}

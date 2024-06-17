use cosmwasm_schema::{QueryResponses, cw_serde};
use cosmwasm_std::{Addr, Binary, BlockInfo, IbcChannel, Uint64};
use cw_utils::Expiration;

use crate::msg::{misc::Order, network::NetworkId};

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
        after_id: Option<ChatMessageId>,
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
    pub messages: Vec<ChatMessageWithId>,
}

#[cw_serde]
pub struct ChatMessage {
    pub user: Addr,
    pub network_id: NetworkId,
    pub message: String,
}

#[cw_serde]
pub struct ChatMessageWithId {
    pub msg: ChatMessage,
    pub id: ChatMessageId,
}

pub type ChatMessageId = Uint64;

pub mod event {
    use cosmwasm_std::{Addr, Event};
    use anyhow::{Error, anyhow};
    use crate::event::CosmwasmEventExt;

    use super::{ChatMessage, ChatMessageWithId};

    /// Event emitted when a new chat message is added
    #[derive(Debug)]
    pub struct ChatMessageEvent {
        pub message: ChatMessageWithId,
    }

    impl ChatMessageEvent {
        pub const KEY: &'static str = "chat-message";
    }

    impl From<ChatMessageEvent> for Event {
        fn from(src: ChatMessageEvent) -> Self {
            let mut event = Event::new(ChatMessageEvent::KEY).add_attributes(vec![
                ("id", src.message.id.to_string()),
                ("user", src.message.msg.user.to_string()),
                ("network-id", src.message.msg.network_id.to_string()),
                ("message", src.message.msg.message),
            ]);

            event
        }
    }

    impl TryFrom<Event> for ChatMessageEvent {
        type Error = Error;

        fn try_from(evt: Event) -> anyhow::Result<Self> {
            if evt.ty.as_str() != format!("wasm-{}", ChatMessageEvent::KEY) {
                return Err(anyhow!("unexpected event type: {}, should be {}", evt.ty, ChatMessageEvent::KEY));
            }

            Ok(ChatMessageEvent {
                message: ChatMessageWithId {
                    msg: ChatMessage {
                        user: Addr::unchecked(evt.string_attr("user")?),
                        network_id: evt.string_attr("network-id")?.parse()?,
                        message: evt.string_attr("message")?,
                    },
                    id: evt.u64_attr("id")?.into(),
                }
            })
        }
    }
}
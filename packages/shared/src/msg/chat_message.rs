use cosmwasm_schema::{QueryResponses, cw_serde};
use cosmwasm_std::{Addr, Binary, BlockInfo, IbcChannel, Uint64};
use cw_utils::Expiration;

use crate::msg::{misc::Order, network::NetworkId};

#[cw_serde]
pub struct ChatMessage {
    pub user: Addr,
    pub network_id: NetworkId,
    pub message: String,
}

#[cw_serde]
pub struct ChatMessageWithIndex {
    pub msg: ChatMessage,
    pub index: ChatMessageIndex,
}

/// This index is NOT a globally unique id
/// it's merely the per-client index of the message to help with pagination
pub type ChatMessageIndex = Uint64;

pub mod event {
    use cosmwasm_std::{Addr, Event};
    use anyhow::{Error, anyhow};
    use crate::event::CosmwasmEventExt;

    use super::{ChatMessage, ChatMessageWithIndex};

    /// Event emitted when a new chat message is added
    #[derive(Debug)]
    pub struct ChatMessageEvent {
        pub message: ChatMessageWithIndex,
    }

    impl ChatMessageEvent {
        pub const KEY: &'static str = "chat-message";
    }

    impl From<ChatMessageEvent> for Event {
        fn from(src: ChatMessageEvent) -> Self {
            let mut event = Event::new(ChatMessageEvent::KEY).add_attributes(vec![
                ("index", src.message.index.to_string()),
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
                message: ChatMessageWithIndex {
                    msg: ChatMessage {
                        user: Addr::unchecked(evt.string_attr("user")?),
                        network_id: evt.string_attr("network-id")?.parse()?,
                        message: evt.string_attr("message")?,
                    },
                    index: evt.u64_attr("index")?.into(),
                }
            })
        }
    }
}
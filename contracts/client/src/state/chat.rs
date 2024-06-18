use cosmwasm_std::{Order, Storage};
use cw_storage_plus::{Bound, Map};
use shared::msg::chat_message::{event::ChatMessageEvent, ChatMessage, ChatMessageIndex, ChatMessageWithIndex};

use super::{State, StateContext};
use anyhow::Result;

const CHAT_MESSAGES:Map<u64, ChatMessage> = Map::new("chat_messages");

impl State<'_> {
    pub fn get_chat_messages(&self, store: &dyn Storage, after_index: Option<ChatMessageIndex>, order: Option<Order>) -> Result<Vec<ChatMessageWithIndex>> {
        CHAT_MESSAGES.range(store, after_index.map(|x| Bound::exclusive(x.u64())), None, order.unwrap_or(Order::Ascending))
            .map(|x| x
                .map(|(index, msg)| ChatMessageWithIndex { msg, index: index.into() })
                .map_err(|err| err.into())
            )
            .collect()
    }

    pub fn push_chat_message(&self, ctx: &mut StateContext, message: ChatMessage) -> Result<ChatMessageIndex> {
        let next_index = CHAT_MESSAGES.keys(ctx.store, None, None, Order::Descending).next().unwrap_or(Ok(0))? + 1;
        
        CHAT_MESSAGES.save(ctx.store, next_index, &message)?;

        let index = next_index.into();

        ctx.response.add_event(ChatMessageEvent {
            message: ChatMessageWithIndex {
                msg: message,
                index,
            }
        });
        Ok(index)
    }
}
use cosmwasm_std::{Order, Storage};
use cw_storage_plus::{Bound, Map};
use shared::msg::contract::client::{event::ChatMessageEvent, ChatMessage, ChatMessageId, ChatMessageWithId};

use super::{State, StateContext};
use anyhow::Result;

const CHAT_MESSAGES:Map<u64, ChatMessage> = Map::new("chat_messages");

impl State<'_> {
    pub fn get_chat_messages(&self, store: &dyn Storage, after_id: Option<ChatMessageId>, order: Option<Order>) -> Result<Vec<ChatMessageWithId>> {
        CHAT_MESSAGES.range(store, after_id.map(|x| Bound::exclusive(x.u64())), None, order.unwrap_or(Order::Ascending))
            .map(|x| x
                .map(|(id, msg)| ChatMessageWithId { msg, id: ChatMessageId::new(id) })
                .map_err(|err| err.into())
            )
            .collect()
    }

    pub fn push_chat_message(&self, ctx: &mut StateContext, message: ChatMessage) -> Result<()> {
        let next_id = CHAT_MESSAGES.keys(ctx.store, None, None, Order::Descending).next().unwrap_or(Ok(0))? + 1;
        
        CHAT_MESSAGES.save(ctx.store, next_id, &message)?;

        ctx.response.add_event(ChatMessageEvent {
            message: ChatMessageWithId {
                msg: message,
                id: next_id.into(),
            }
        });
        Ok(())
    }
}
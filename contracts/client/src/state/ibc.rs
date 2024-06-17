use cosmwasm_std::{
    from_binary, from_json, to_json_binary, IbcChannel, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcMsg, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcTimeout, Order, Storage
};
use cw_storage_plus::{Deque, Item, Map};
use shared::{ibc::{
    event::{IbcChannelCloseEvent, IbcChannelConnectEvent},
    validate_ibc_channel_order_and_version, TIMEOUT_SECONDS,
}, msg::ibc::IbcExecuteMsg};
use anyhow::Result;

use super::{State, StateContext};

const SERVER_CHANNEL: Item<IbcChannel> = Item::new("server");

impl State<'_> {

    pub fn get_server_channel(&self, store: &dyn Storage) -> Result<Option<IbcChannel>> {
        SERVER_CHANNEL.may_load(store).map_err(|err| err.into())
    }

    pub fn handle_ibc_channel_open(&self, msg: IbcChannelOpenMsg) -> Result<()> {
        validate_ibc_channel_order_and_version(msg.channel(), msg.counterparty_version())?;
        Ok(())
    }

    pub fn handle_ibc_channel_connect(
        &self,
        ctx: &mut StateContext,
        msg: IbcChannelConnectMsg,
    ) -> Result<()> {
        let channel = msg.channel();

        validate_ibc_channel_order_and_version(channel, msg.counterparty_version())?;

        SERVER_CHANNEL.save(ctx.store, channel)?;

        ctx.response_mut()
            .add_event(IbcChannelConnectEvent { channel });

        Ok(())
    }

    pub fn handle_ibc_channel_close(
        &self,
        ctx: &mut StateContext,
        msg: IbcChannelCloseMsg,
    ) -> Result<()> {
        let channel = msg.channel();
        SERVER_CHANNEL.remove(ctx.store);

        ctx.response_mut()
            .add_event(IbcChannelCloseEvent { channel });
        Ok(())
    }

    pub fn handle_ibc_packet_receive(
        &self,
        ctx: &mut StateContext,
        recv_msg: IbcPacketReceiveMsg,
    ) -> Result<()> {
        from_json(&recv_msg.packet.data)
            .map_err(|err| err.into())
            .and_then(|msg| {
                match msg {
                    IbcExecuteMsg::SendMessageToClient{ message } => {
                        self.push_chat_message(ctx, message)?;
                        Ok(())
                    },
                    _ => anyhow::bail!("unsupported message type"),
                }
            })
    }

    pub fn handle_ibc_packet_ack(&self, _ack: IbcPacketAckMsg) -> Result<()> {
        // Nothing to do here. We don't keep any state about the other
        // chain, just deliver messages so nothing to update.
        //
        // If we did care about how the other chain received our message
        // we could deserialize the data field into an `Ack` and inspect
        // it.
        Ok(())
    }

    pub fn handle_ibc_packet_timeout(&self, _msg: IbcPacketTimeoutMsg) -> Result<()> {
        // As with ack above, nothing to do here. If we cared about
        // keeping track of state between the two chains then we'd want to
        // respond to this likely as it means that the packet in question
        // isn't going anywhere.
        Ok(())
    }
}
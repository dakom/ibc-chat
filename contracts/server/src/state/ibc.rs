use cosmwasm_std::{
    from_binary, from_json, to_json_binary, IbcChannel, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcEndpoint, IbcMsg, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcTimeout, Order, Storage
};
use cw_storage_plus::{Deque, Item, Map};
use shared::{ibc::{
    event::{IbcChannelCloseEvent, IbcChannelConnectEvent},
    validate_ibc_channel_order_and_version, TIMEOUT_SECONDS,
}, msg::{chat_message::event::ChatMessageEvent, ibc::IbcExecuteMsg}};
use anyhow::Result;

use super::{State, StateContext};

// TODO - make newtype with all the impls
type IbcChannelKey = String;

// keyed by the counterside port id
const CLIENT_CHANNELS: Map<IbcChannelKey, IbcChannel> = Map::new("clients");

fn channel_to_key(channel: &IbcChannel) -> IbcChannelKey {
    format!("{}-{}", channel.endpoint.port_id, channel.endpoint.channel_id)
}

impl State<'_> {

    pub fn get_client_channels(&self, store: &dyn Storage) -> Result<Vec<IbcChannel>> {
        CLIENT_CHANNELS.range(store, None, None, Order::Ascending)
            .map(|x| x
                .map(|(_, channel)| channel)
                .map_err(|err| err.into())
            )
            .collect()
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

        let key = channel_to_key(&channel); 

        if CLIENT_CHANNELS.has(ctx.store, key.clone()) {
            anyhow::bail!("channel for {} already exists", key);
        }

        CLIENT_CHANNELS.save(ctx.store, key, channel)?;

        ctx.response
            .add_event(IbcChannelConnectEvent { channel });

        Ok(())
    }

    pub fn handle_ibc_channel_close(
        &self,
        ctx: &mut StateContext,
        msg: IbcChannelCloseMsg,
    ) -> Result<()> {
        let channel = msg.channel();
        CLIENT_CHANNELS.remove(ctx.store, channel_to_key(&channel));

        ctx.response
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
                    IbcExecuteMsg::SendMessageToServer{ message } => {
                        let mut response_messages = Vec::new();
                        for item in CLIENT_CHANNELS.range(ctx.store, None, None, Order::Ascending) { 
                            let (_, channel) = item?;
                            if channel.counterparty_endpoint != recv_msg.packet.src {

                                response_messages.push(IbcMsg::SendPacket {
                                    channel_id: channel.endpoint.channel_id,
                                    data: to_json_binary(&IbcExecuteMsg::SendMessageToClient { message: message.msg.clone() })?,
                                    timeout: IbcTimeout::with_timestamp(self.env.block.time.plus_seconds(TIMEOUT_SECONDS)),
                                });
                            }
                        }

                        ctx.response.add_event(ChatMessageEvent {
                            message
                        });

                        for response_message in response_messages {
                            ctx.response.add_message(response_message);
                        }
                        Ok(())
                    },
                    _ => {
                        anyhow::bail!("unsupported message type")
                    }
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
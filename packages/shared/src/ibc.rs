//! Ibc helpers
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, Binary, IbcChannel, IbcOrder};
use anyhow::{Result, bail};

/// Timeout in seconds for IBC packets
pub const TIMEOUT_SECONDS: u64 = 60 * 2; // 2 minutes

pub fn validate_ibc_channel_order_and_version(
    channel: &IbcChannel,
    counterparty_version: Option<&str>,
) -> Result<()> {
    // We expect an unordered channel here. Ordered channels have the
    // property that if a message is lost the entire channel will stop
    // working until you start it again.
    if channel.order != IbcOrder::Unordered {
        bail!("Only unordered channels are supported for this demo");
    }

    // Make sure that we're talking with a counterparty who speaks the
    // same "protocol" as us.
    //
    // For a connection between chain A and chain B being established
    // by chain A, chain B knows counterparty information during
    // `OpenTry` and chain A knows counterparty information during
    // `OpenAck`. We verify it when we have it but when we don't it's
    // alright.
    if let Some(counterparty_version) = counterparty_version {
        if counterparty_version != channel.version {
            bail!(
                "wrong ibc counterparty version, expected {} got {}",
                channel.version,
                counterparty_version,
            );
        }
    }

    Ok(())
}

/// IBC ACK. See:
/// https://github.com/cosmos/cosmos-sdk/blob/f999b1ff05a4db4a338a855713864497bedd4396/proto/ibc/core/channel/v1/channel.proto#L141-L147
#[cw_serde]
pub enum IbcAck {
    Success,
    // error must be a string
    Error(String),
}

/// Common IBC Events
pub mod event {
    use cosmwasm_std::{Event, IbcChannel};

    /// IBC Channel Connect Event
    #[derive(Debug)]
    pub struct IbcChannelConnectEvent<'a> {
        /// The IBC channel
        pub channel: &'a IbcChannel,
    }

    impl<'a> From<IbcChannelConnectEvent<'a>> for Event {
        fn from(src: IbcChannelConnectEvent) -> Self {
            mixin_ibc_channel(Event::new("ibc-channel-connect"), src.channel)
        }
    }

    /// IBC Channel Close Event
    #[derive(Debug)]
    pub struct IbcChannelCloseEvent<'a> {
        /// The IBC channel
        pub channel: &'a IbcChannel,
    }

    impl<'a> From<IbcChannelCloseEvent<'a>> for Event {
        fn from(src: IbcChannelCloseEvent) -> Self {
            mixin_ibc_channel(Event::new("ibc-channel-close"), src.channel)
        }
    }

    fn mixin_ibc_channel(event: Event, channel: &IbcChannel) -> Event {
        event
            .add_attribute("endpoint-id", &channel.endpoint.channel_id)
            .add_attribute("endpoint-port-id", &channel.endpoint.port_id)
            .add_attribute(
                "counterparty-endpoint-id",
                &channel.counterparty_endpoint.channel_id,
            )
            .add_attribute(
                "counterparty-endpoint-port-id",
                &channel.counterparty_endpoint.port_id,
            )
            .add_attribute("order", format!("{:?}", channel.order))
            .add_attribute("version", &channel.version)
            .add_attribute("connection-id", &channel.connection_id)
    }
}
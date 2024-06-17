use std::collections::{HashMap, HashSet};

use cosmwasm_std::Order;
use shared::msg::contract::client::{ChatMessagesResp, InfoResp as ClientInfoResp};
use crate::{clone, helpers::try_until_ibc_timeout, prelude::*};

pub async fn run_all_tests() -> Result<()> {

    test_channel_creation().await?;
    test_message_broadcast().await?;

    log::info!("All tests succeeded!");

    Ok(())
}

// Just tests to see that each client has a channel connected
async fn test_channel_creation() -> Result<()> {
    let server_info:ServerInfoResp = Wallet::server().into_contract_server().query_info().await.unwrap();

    if server_info.client_channels.len() != Wallet::all_clients().len() {
        anyhow::bail!("server does not have all clients connected");
    }

    for wallet in Wallet::all_clients() {
        let network_id = wallet.network_id();
        let mut client_contract = wallet.into_contract_client();

        let client_info:ClientInfoResp = client_contract.query_info().await.unwrap();

        let client_channel = client_info.server_channel.context(format!("server channel is none for {}", network_id))?;

        let client_server_channel_match = server_info
            .client_channels
            .iter()
            .any(|server_channel| {
                server_channel.counterparty_endpoint == client_channel.endpoint
                && server_channel.endpoint == client_channel.counterparty_endpoint
            });

        if !client_server_channel_match {
            anyhow::bail!("client {} does not have a channel to server", network_id);
        }

        log::info!("Client {} is connected to server", network_id);
    }

    log::info!("Channels are all set up!");

    Ok(())
}

// This test does the following:
// 1. Send a message on each client
// 2. Poll for new messages on each client until we have seen all messages from all other clients
//
// If we don't get all messages within the timeout, this will fail
async fn test_message_broadcast() -> Result<()> {
    for wallet in Wallet::all_clients() {
        let network_id = wallet.network_id();
        log::info!("");
        log::info!("---- Testing broadcast on {} ----", network_id);
        log::info!("");


        let mut client_contract = wallet.into_contract_client();

        let ChatMessagesResp{messages: messages_before}  = client_contract.query_chat_messages(None, Some(Order::Descending)).await?;

        let mut message_cursor = messages_before.first().map(|m| m.id);
        let mut waiting_network_ids = HashSet::new();

        for other_wallet in Wallet::all_clients() {
            if other_wallet.network_id() == network_id {
                continue;
            }

            let other_network_id = other_wallet.network_id();
            waiting_network_ids.insert(other_network_id);
            let mut other_client_contract = other_wallet.into_contract_client();
            let resp = other_client_contract.exec_send_message(&format!("hello from {}", other_network_id)).await?;

            log::info!("Sent from {}", other_network_id);
        }

        loop {
            let waiting_len = waiting_network_ids.len();
            // poll for new messages - if there are none within the timeout, then it's definitely an error
            // since we know we sent a message
            // and we only loop if we are waiting for messages
            let new_messages = try_until_ibc_timeout(client_contract.clone(), |client_contract| async move {
                let mut client_contract = client_contract;

                let ChatMessagesResp{messages: messages_after}  = client_contract.query_chat_messages(message_cursor, Some(Order::Ascending)).await.unwrap();
                if messages_after.is_empty() {
                    log::info!("No new messages on {}", network_id);
                }

                if messages_after.is_empty() {
                    None
                } else {
                    Some(messages_after)
                }
            }).await?;

            message_cursor = new_messages.last().map(|m| m.id);

            let got_new_messages = !new_messages.is_empty();

            for message in new_messages {
                if waiting_network_ids.remove(&message.msg.network_id) {
                    log::info!("Got new chat message: (#{}) {}", message.id, message.msg.message);
                } else if message.msg.network_id == network_id {
                    // this is unexpected, since we only sent messages from other networks
                    // but it isn't necessarily an error with maybe some old packets or something...
                    log::warn!("Got echo message: (#{}) {}", message.id, message.msg.message);
                }
            }

            if got_new_messages {
                log::info!("{} more to go!", waiting_network_ids.len());
            }

            // got all messages, done for this network!
            if waiting_network_ids.is_empty() {
                break;
            }
        }
    }

    log::info!("Message broadcast succeeded!");

    Ok(())
}

use gloo_timers::future::TimeoutFuture;
use shared::{event::CosmwasmEventExt, msg::{chat_message::event::ChatMessageEvent, contract::client::ChatMessagesResp}};
use wallet::wallet_contract_impls::{WalletSigningContractClient, WalletSigningContractServer};
use wasm_bindgen_futures::spawn_local;

use crate::{config::CONFIG, page::chat::{display::ChatDisplay, window::{chat_window_label_render, WINDOW_CLASS}}, prelude::*};
pub(super) struct ChatWindowServer {
    contract: WalletSigningContractServer,
    display: ChatDisplay
}

impl ChatWindowServer {
    pub fn new(contract: WalletSigningContractServer) -> Arc<Self> {
        Arc::new(Self {
            contract,
            display: ChatDisplay::new(ContractKind::Server)
        })
    }

    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self;
        html!("div", {
            .future(clone!(state => async move {
                state.poll_events().await;
            }))
            .class(&*WINDOW_CLASS)
            .child(chat_window_label_render(state.contract.wallet.network_id(), ContractKind::Server))
            .child(self.display.render())
        })
    }

    async fn poll_events(self: &Arc<Self>) {
        let state = self;

        // get first height
        let mut height = loop {
            if let Ok(curr_height) = state.contract.wallet.get_height().await {
                break curr_height;
            }
            TimeoutFuture::new(CONFIG.events_poll_delay_ms).await;
        };

        loop {
            let mut messages = Vec::new();
            loop {
                let curr_height = state.contract.wallet.get_height().await.unwrap();
                if height == curr_height {
                    break;
                }
                let txs = state.contract.wallet.search_tx(&format!("tx.height={}", height)).await.unwrap();
                for tx in txs {
                    for raw_event in tx.events {
                        if let Ok(kind) = raw_event.string_attr("contract_kind") {
                            if kind == ContractKind::Server.to_string() {
                                if let Ok(event) = ChatMessageEvent::try_from(raw_event) {
                                    messages.push(event.message);
                                } 
                            }
                        }
                    }
                }
                height += 1;
            }

            if !messages.is_empty() {
                state.display.add_messages(messages);
            }

            // sleep for a bit
            TimeoutFuture::new(CONFIG.events_poll_delay_ms).await;
        }
    }
}
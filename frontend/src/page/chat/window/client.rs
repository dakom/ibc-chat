
use gloo_timers::future::TimeoutFuture;
use shared::msg::{chat_message::event::ChatMessageEvent, contract::client::ChatMessagesResp};
use wallet::wallet_contract_impls::{WalletSigningContractClient, WalletSigningContractServer};
use wasm_bindgen_futures::spawn_local;

use crate::{config::CONFIG, page::chat::{display::ChatDisplay, input::ChatInput, window::{chat_window_label_render, WINDOW_CLASS}}, prelude::*};
pub(super) struct ChatWindowClient {
    contract: WalletSigningContractClient,
    display: ChatDisplay,
    input: Arc<ChatInput>,
}

impl ChatWindowClient {
    pub fn new(contract: WalletSigningContractClient) -> Arc<Self> {
        Arc::new(Self {
            contract,
            display: ChatDisplay::new(ContractKind::Client),
            input: ChatInput::new(),
        })
    }

    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self;
        html!("div", {
            .future(clone!(state => async move {
                state.poll_messages().await;
            }))
            .class(&*WINDOW_CLASS)
            .child(chat_window_label_render(state.contract.wallet.network_id(), ContractKind::Client))
            .child(state.display.render())
            .child(state.input.render(clone!(state => move |text| {
                spawn_local(clone!(state => async move {
                    if let Err(e) = state.contract.clone().exec_send_message(text).await {
                        web_sys::window().unwrap().alert_with_message(&format!("Error sending message: {:?}", e));
                    }
                }));
            })))
        })
    }

    async fn poll_messages(self: &Arc<Self>) {
        let state = self;
        let mut message_cursor = None;
        loop {
            let ChatMessagesResp {messages} = state.contract.clone().query_chat_messages(message_cursor, None).await.unwrap();
            if !messages.is_empty() {
                message_cursor = Some(messages.last().unwrap().index);
                state.display.add_messages(messages);
            }

            // sleep for a bit
            TimeoutFuture::new(CONFIG.messages_poll_delay_ms).await;
        }
    }

}
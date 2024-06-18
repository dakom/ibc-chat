mod client;
mod server;

use client::ChatWindowClient;
use gloo_timers::future::TimeoutFuture;
use server::ChatWindowServer;
use shared::msg::{contract::client::ChatMessagesResp, network::NetworkId};
use wallet::wallet_contract_impls::{WalletSigningContractClient, WalletSigningContractServer};
use wasm_bindgen_futures::spawn_local;

use crate::{config::CONFIG, prelude::*};

use super::{
    display::ChatDisplay,
    input::ChatInput,
};

pub struct ChatWindow {
    contract: ChatWindowContract
}

impl ChatWindow {
    pub fn new(contract: ChatWindowContract) -> Self {
        Self {
            contract
        }
    }

    pub fn render(&self) -> Dom {
        match self.contract.clone() {
            ChatWindowContract::Client(client) => ChatWindowClient::new(client).render(), 
            ChatWindowContract::Server(server) => ChatWindowServer::new(server).render(), 
        }
    }
}

static WINDOW_CLASS:Lazy<String> = Lazy::new(|| {
    class! {
        .style("--horizontal-padding", "1rem")
        .style("--vertical-padding", "1rem")
        .style("border", "1px solid black")
        .style("background-color", "whitesmoke")
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("--container-width", "calc(50vw - var(--horizontal-padding))")
        .style("--container-height", "calc(50vh - var(--vertical-padding))")
        .style("width", "var(--container-width)")
        .style("height", "var(--container-height)")
    }
});


fn chat_window_label_render(network_id: NetworkId, contract_kind: ContractKind) -> Dom {
    static CLASS:Lazy<String> = Lazy::new(|| {
        class! {
            .style("padding-left", "1rem")
            .style("text-align", "center")
        }
    });
    html!("div", {
        .class(&*CLASS)
        .class(&*TEXT_SIZE_XLG)
        .text(&match contract_kind {
            ContractKind::Client => format!("{} (client)", network_id),
            ContractKind::Server => format!("{} (server - events)", network_id),
        })
    })
}

// just a helper to pass in config
#[derive(Clone)]
pub enum ChatWindowContract {
    Client(WalletSigningContractClient),
    Server(WalletSigningContractServer),
}

impl From<WalletSigningContractClient> for ChatWindowContract {
    fn from(contract: WalletSigningContractClient) -> Self {
        Self::Client(contract)
    }
}

impl From<WalletSigningContractServer> for ChatWindowContract {
    fn from(contract: WalletSigningContractServer) -> Self {
        Self::Server(contract)
    }
}
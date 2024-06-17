use shared::msg::contract::server::InfoResp as ServerInfoResp;
use shared::msg::contract::client::InfoResp as ClientInfoResp;
use crate::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{atoms::{balance::Balance, buttons::Squareish1Button, sidebar::Sidebar}, config::CONFIG, prelude::*, route::Route};

pub struct ChatPage {
    pub info: Mutable<String>,
}

impl ChatPage {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            info: Mutable::new("".to_string()),
        })
    }

    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self;
        static CONTAINER:Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "grid")
                .style("grid-template-columns", "auto 1fr")
            }
        });
        html!("div", {
            .class(&*CONTAINER)
            .future(clone!(state => async move {
                let neutron:ServerInfoResp = Wallet::neutron().into_contract_server().query_info().await.unwrap_ext();
                let stargaze:ClientInfoResp = Wallet::stargaze().into_contract_client().query_info().await.unwrap_ext();
                let kujira:ClientInfoResp = Wallet::kujira().into_contract_client().query_info().await.unwrap_ext();
                let nois:ClientInfoResp = Wallet::nois().into_contract_client().query_info().await.unwrap_ext();

                log::info!("{:#?}", neutron);
                log::info!("{:#?}", stargaze);
                log::info!("{:#?}", kujira);
                log::info!("{:#?}", nois);

                state.info.set_neq(format!("neutron: {:#?} stargaze: {:#?} kujira: {:#?} nois: {:#?}", neutron, stargaze, kujira, nois));

            }))
            .text_signal(state.info.signal_cloned().map(|info| info))
            .child(Squareish1Button::new().render("Send Message".to_string(), clone!(state => move || {
                spawn_local(async {
                    let res = Wallet::kujira().into_contract_client().exec_send_message("hello world").await.unwrap();

                    log::info!("res: {:#?}", res);
                })
            })))
        })
    }
}
mod window;
mod display;
mod input;

use shared::msg::contract::server::InfoResp as ServerInfoResp;
use shared::msg::contract::client::InfoResp as ClientInfoResp;
use window::ChatWindow;
use crate::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{atoms::{balance::Balance, buttons::Squareish1Button, sidebar::Sidebar}, config::CONFIG, prelude::*, route::Route};

pub struct ChatPage {
    windows: Vec<ChatWindow>,
}

impl ChatPage {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            windows: vec![
                ChatWindow::new(Wallet::kujira().into_contract_client().into()),
                ChatWindow::new(Wallet::nois().into_contract_client().into()),
                ChatWindow::new(Wallet::stargaze().into_contract_client().into()),
                ChatWindow::new(Wallet::neutron().into_contract_server().into()),
            ]
        })
    }

    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self;
        static CONTAINER:Lazy<String> = Lazy::new(|| {
            class! {
                .style("width", "100vw")
                .style("height", "100vh")
                .style("background-color", "rgb(49, 49, 49)")
            }
        });
        static GRID:Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "grid")
                .style("grid-template-columns", "1fr 1fr")
                .style("grid-template-rows", "1fr 1fr")
                .style("width", "100%")
                .style("height", "100%")
            }
        });

        static CELL:Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "flex")
                .style("justify-content", "center")
                .style("align-items", "center")
            }
        });
        html!("div", {
            .class(&*CONTAINER)
            .child(html!("div", {
                .class(&*GRID)
                .children(state.windows.iter().map(|window| html!("div", {
                    .class(&*CELL)
                    .child(window.render())
                })))
            }))
        })
        // .future(clone!(state => async move {
        //     let neutron:ServerInfoResp = Wallet::neutron().into_contract_server().query_info().await.unwrap_ext();
        //     let stargaze:ClientInfoResp = Wallet::stargaze().into_contract_client().query_info().await.unwrap_ext();
        //     let kujira:ClientInfoResp = Wallet::kujira().into_contract_client().query_info().await.unwrap_ext();
        //     let nois:ClientInfoResp = Wallet::nois().into_contract_client().query_info().await.unwrap_ext();

        //     log::info!("{:#?}", neutron);
        //     log::info!("{:#?}", stargaze);
        //     log::info!("{:#?}", kujira);
        //     log::info!("{:#?}", nois);

        //     state.info.set_neq(format!("neutron: {:#?} stargaze: {:#?} kujira: {:#?} nois: {:#?}", neutron, stargaze, kujira, nois));

        // }))
        // .text_signal(state.info.signal_cloned().map(|info| info))
        // .child(Squareish1Button::new().render("Send Message".to_string(), clone!(state => move || {
        //     spawn_local(async {
        //         let res = Wallet::kujira().into_contract_client().exec_send_message("hello world").await.unwrap();

        //         log::info!("res: {:#?}", res);
        //     })
        // })))
    }
}
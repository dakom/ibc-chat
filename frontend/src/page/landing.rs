use wallet::get_cosmjs_web;

use crate::{atoms::buttons::Squareish1Button, config::{CHAINENV, CONFIG}, page::chat::ChatPage, prelude::*, route::Route};

pub struct Landing {
    wallet_connected: Mutable<bool>,
}

impl Landing {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            wallet_connected: Mutable::new(false),
        })
    }

    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self;

        html!("div", {
            .style("height", "100%")
            .future(state.wallet_connected.signal().for_each(|connected| {
                async move {
                    // for debugging, we want to jump to an initial page, but:
                    // 1. only consider it after connection status has settled
                    // 2. only one time (not if we intentionally come back to landing)
                    if connected {
                        let start_route = CONFIG.start_route.lock().unwrap_ext().take();
                        log::info!("Starting at route: {:?}", start_route);
                        if let Some(start_route) = start_route {
                            start_route.go_to_url();
                        }
                    }
                }
            }))
            .child(html!("div", {
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("justify-content", "center")
                .style("align-items", "center")
                .style("height", "100%")
                .style("gap", "1rem")
                .child(html!("div", {
                    .class(&*TEXT_SIZE_XLG)
                    .style("font-weight", "bold")
                    .style("text-align", "center")
                    .text(&get_text!("landing-title"))
                }))
                .child(html!("img", {
                    .style("width", "10rem")
                    .style("height", "10rem")
                    .attribute("src", &CONFIG.app_image_url("w3bay-logo.jpg"))
                }))
                .child(html!("div", {
                    .style("margin-top", "2rem")
                    .child_signal(state.wallet_connected.signal().map(clone!(state => move |connected| {
                        Some(
                            if connected {
                                state.render_choose_persona()
                            } else {
                                state.render_connect_button()
                            }
                        )
                    })))
                }))
            }))
        })
    }

    fn render_connect_button(self: &Arc<Self>) -> Dom {
        let state = self;
        #[derive(Clone, Copy, Debug, PartialEq)]
        enum Phase {
            Init,
            Connecting,
            NoKeplr,
            InstallingKeplr,
        }

        let phase = Mutable::new(Phase::Init);

        html!("div", {
            .future(phase.signal().for_each(clone!(state, phase => move |phase_value| {
                clone!(state, phase => async move {
                    match phase_value {
                        Phase::Init => {
                            if CONFIG.auto_connect {
                                phase.set_neq(Phase::Connecting);
                            }
                        },
                        Phase::Connecting => {
                            match Wallet::connect(get_cosmjs_web(), *CHAINENV, None).await {
                                Ok(instance) => {
                                    state.wallet_connected.set(true);
                                },
                                Err(_) => {
                                    phase.set(Phase::NoKeplr)
                                }
                            }
                        },
                        Phase::NoKeplr => {
                            // could call ffi_install_keplr...
                        }
                        Phase::InstallingKeplr => {
                            Wallet::install_keplr(*CHAINENV).await;
                            phase.set(Phase::Init);
                        }
                    }
                })
            })))
            .style("display", "flex")
            .style("justify-content", "center")
            .style("align-items", "center")
            .style("gap", "1rem")
            .child_signal(phase.signal().map(clone!(phase => move |phase_value| {
                Some(match phase_value {
                    Phase::Init => {
                        Squareish1Button::new()
                            .render(get_text!("landing-wallet-connect"), clone!(phase => move || {
                                phase.set_neq(Phase::Connecting);
                            }))
                    },
                    Phase::Connecting => {
                        html!("div", {
                            .class(&*TEXT_SIZE_LG)
                            .text("Connecting...")
                        })
                    },
                    Phase::NoKeplr => {
                        html!("div", {
                            .style("display", "flex")
                            .style("flex-direction", "column")
                            .style("gap", "1rem")
                            .style("align-items", "center")

                            .child(html!("div", {
                                .class(&*TEXT_SIZE_LG)
                                .text("Unable to connect")
                            }))
                            .child(Squareish1Button::new()
                                .render("install chain for Keplr".to_string(), clone!(phase => move || {
                                    phase.set_neq(Phase::InstallingKeplr);
                                }))
                            )
                        })
                    },
                    Phase::InstallingKeplr => {
                        html!("div", {
                            .class(&*TEXT_SIZE_LG)
                            .text("Installing Keplr...")
                        })
                    },
                })
            })))
        })
    }

    fn render_choose_persona(self: &Arc<Self>) -> Dom {
        html!("div", {
            .style("display", "flex")
            .style("justify-content", "center")
            .style("align-items", "center")
            .style("gap", "1rem")
            .child(html!("div", {
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("align-items", "center")
                .child(html!("div", {
                    .style("display", "flex")
                    .style("gap", "1rem")
                    .style("margin-bottom", "1rem")
                    .child(Squareish1Button::new()
                        .render(get_text!("landing-start-chat"), || {
                            Route::Chat.go_to_url()
                        }) 
                    )
                }))
                // .child(html!("div", {
                //     .class(&*TEXT_SIZE_LG)
                //     .style("text-align", "center")
                //     .style("margin-bottom", "1rem")
                //     .text(&format!("{}", Wallet::neutron().address()))
                // }))
                // .child(html!("div", {
                //     .class(&*TEXT_SIZE_LG)
                //     .style("text-align", "center")
                //     .style("margin-bottom", "1rem")
                //     .text(&format!("{}", Wallet::kujira().address()))
                // }))
                // .child(html!("div", {
                //     .class(&*TEXT_SIZE_LG)
                //     .style("text-align", "center")
                //     .style("margin-bottom", "1rem")
                //     .text(&format!("{}", Wallet::stargaze().address()))
                // }))
            }))
        })
    }

}
use awsm_web::loaders::helpers::{spawn_handle, FutureHandle};
use shared_dev_tools::status::ContractStatusEvent;
use wallet::config::Environment;
use wasm_bindgen_futures::spawn_local;

use crate::{atoms::{buttons::ButtonSize, dynamic_svg::color_circle::ColorCircle}, prelude::*};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
enum ContractStatus {
    Init,
    NeedsBuildAndDeploy,
    UpToDate,
}

pub struct ContractStatusUi {
    pub status: Mutable<ContractStatus>,
    pub callback: RwLock<Option<Closure<dyn FnMut(JsValue)>>>,
}

impl ContractStatusUi {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            status: Mutable::new(ContractStatus::Init),
            callback: RwLock::new(None),
        })
    }

    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self;

        static CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "flex")
                .style("align-items", "center")
                .style("gap", "1rem")
            }
        });

        html!("div", {
            .future(clone!(state => async move {
                state.init_status().await.unwrap_ext();
                state.watch_status().await.unwrap_ext();
            }))
            .after_removed(|_| {
                spawn_local(async {
                    tauri::stop_contract_status_watcher().await.unwrap_ext();
                })
            })
            .class(&*CLASS)
            .class(&*TEXT_SIZE_MD)
            .child(html!("div", {
                .text("Contract Status:")
            }))
            .child_signal(state.status.signal_cloned().map(|status| {
                Some(html!("div", {
                    .text(match status {
                        ContractStatus::Init => "Loading...",
                        ContractStatus::NeedsBuildAndDeploy => "Out of date",
                        ContractStatus::UpToDate => "Up to date",
                    })
                }))
            }))
            .child(ColorCircle::render(ButtonSize::Sm, state.status.signal_cloned().map(|status| {
                match status {
                    ContractStatus::Init => ColorSemantic::MidGrey,
                    ContractStatus::NeedsBuildAndDeploy => ColorSemantic::Warning,
                    ContractStatus::UpToDate => ColorSemantic::Success,
                }
            })))
        })
    }

    async fn watch_status(self: &Arc<Self>) -> Result<()> {
        let state = self;
        // Setup the watcher for updating when contracts change
        let callback = tauri::start_contract_status_watcher(
            SETTINGS.env.get(), 
            clone!(state => move |event| {
                let acc = state.status.get();
                let status = event.current_deployed.into_iter().fold(acc, |acc, (network_id, deployed)| {
                    if deployed {
                        acc
                    } else {
                        ContractStatus::NeedsBuildAndDeploy
                    }
                });
                state.status.set_neq(status);
            })
        ).await?;

        // keep the callback alive
        *state.callback.write().unwrap_ext() = Some(callback);

        Ok(())
    }

    async fn init_status(self: &Arc<Self>) -> Result<()> {
        let state = self;

        // get the initial statuses to set initial state
        let statuses = tauri::get_contract_status(SETTINGS.env.get()).await?;

        let status = statuses.into_iter().fold(ContractStatus::UpToDate, |acc, event| {
            event.current_deployed.into_iter().fold(acc, |acc, (network_id, deployed)| {
                if deployed {
                    acc
                } else {
                    ContractStatus::NeedsBuildAndDeploy
                }
            })
        });

        state.status.set_neq(status);

        Ok(())
    }
}


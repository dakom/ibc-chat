use awsm_web::loaders::helpers::{spawn_handle, FutureHandle};
use shared_dev_tools::{process::{ProcessId, ProcessKind}, status::ContractStatusEvent};
use wallet::config::Environment;
use wasm_bindgen_futures::spawn_local;

use crate::{atoms::{buttons::{ButtonSize, Squareish1Button}, dynamic_svg::color_circle::ColorCircle}, config::CONFIG, page::logs::{LogItemOptions, LOGS}, prelude::*};

pub struct ContractDeployUi {
}

impl ContractDeployUi {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
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
            .child(Squareish1Button::new().set_size(ButtonSize::Md).render("Deploy".to_string(), || {
                spawn_local(async move {
                    let process_id = tauri::start_process(ProcessKind::DeployContracts, SETTINGS.env.get()).await.unwrap_ext();

                    let log_id = LOGS.add_item(LogItemOptions {
                        title: "Deploying Contracts".to_string(), 
                        color: Some(ColorSemantic::Warning),
                        process_id: Some(process_id)
                    });

                    for i in 0..100 {
                        LOGS.add_line(log_id, format!("({i}) Here's a line of text")).unwrap_ext();
                    }
                });
            }))
        })
    }
}
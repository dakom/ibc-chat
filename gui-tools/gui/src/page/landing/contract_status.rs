use shared_gui::status::ContractStatus;

use crate::{atoms::dynamic_svg::color_circle::ColorCircle, prelude::*};

pub struct ContractStatusUi {
    pub status: Mutable<ContractStatus>,
}

impl ContractStatusUi {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            status: Mutable::new(ContractStatus::Init),
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
                // TODO - poll!

                let status = tauri::contract_status().await.unwrap();
                state.status.set_neq(status);
            }))
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
            .child(ColorCircle::render(state.status.signal_cloned().map(|status| {
                match status {
                    ContractStatus::Init => ColorSemantic::MidGrey,
                    ContractStatus::NeedsBuildAndDeploy => ColorSemantic::Warning,
                    ContractStatus::UpToDate => ColorSemantic::Success,
                }
            })))
        })
    }
}


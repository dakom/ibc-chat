mod contract_status;
mod contract_deploy;

use contract_deploy::ContractDeployUi;
use contract_status::ContractStatusUi;

use crate::{atoms::{buttons::Squareish1Button, image::render_app_img}, config::CONFIG, prelude::*, route::Route};


pub struct DashboardUi {
}

struct DropLogger {
}

impl Drop for DropLogger {
    fn drop(&mut self) {
        log::info!("dropped!");
    }
}

impl DashboardUi {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
        })
    }

    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self;

        static CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("padding", "1rem")
            }
        });
        html!("div", {
            .class(&*CLASS)
            .child(html!("div", {
                .style("display", "flex")
                .style("gap", "1rem")

                .child(ContractStatusUi::new().render())
                .child(ContractDeployUi::new().render())
            }))
        })
    }
}
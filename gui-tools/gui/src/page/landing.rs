mod contract_status;

use contract_status::ContractStatusUi;

use crate::{atoms::buttons::Squareish1Button, config::CONFIG, prelude::*, route::Route};


pub struct Landing {
}

impl Landing {
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
                .child(ContractStatusUi::new().render())
            }))
        })
    }
}
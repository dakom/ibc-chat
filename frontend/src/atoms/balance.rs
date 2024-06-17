use futures::StreamExt;
use gloo_timers::future::IntervalStream;
use crate::prelude::*;
pub struct Balance {
    label: String,
    balance: Mutable<f64>,
    denom: String
}

impl Balance {
    pub fn new(label: String) -> Arc<Self> {
        Arc::new(Self {
            label,
            balance: Mutable::new(0.0),
            denom: Self::wallet().denom()
        })
    }

    fn wallet() -> WalletSigning {
        Wallet::kujira()
    }

    pub fn render(self: Arc<Self>) -> Dom {
        let state = self;

        html!("div", {
            .future(clone!(state => async move {
                state.balance.set_neq(Self::wallet().balance().await.unwrap());
            }))
            .class(&*TEXT_SIZE_XLG)
            .child(html!("div", {
                .text_signal(state.balance.signal().map(clone!(state => move |balance| format!("{} - Balance: {:.2}{}", state.label, balance, state.denom))))
            }))
            .child(html!("div", {
                .future(IntervalStream::new(3_000).for_each(clone!(state => move |_| clone!(state => async move {
                    state.balance.set_neq(Self::wallet().balance().await.unwrap());
                }))))
            }))
        })
    }
}
use wallet::config::Environment;
use crate::atoms::dropdown::Dropdown;
use crate::prelude::*;

pub struct Settings {
    pub env: Mutable<Environment>
}

pub static SETTINGS:Lazy<Settings> = Lazy::new(|| {
    Settings {
        env: Mutable::new(Environment::Local)
    }
});


use crate::{atoms::buttons::Squareish1Button, config::CONFIG, prelude::*, route::Route};


pub struct SettingsUi {
}

impl SettingsUi {
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
            .child(state.render_environment())
        })
    }

    fn render_environment(self: &Arc<Self>) -> Dom {
        let state = self;

        Dropdown::new(
            "Environment".to_string(), 
            Some(Environment::Local),
            vec![
                (Environment::Local.as_str().to_string(), Environment::Local),
                (Environment::Testnet.as_str().to_string(), Environment::Testnet),
            ]
        ).render(|env| {
            SETTINGS.env.set_neq(*env);
        })
    }
}
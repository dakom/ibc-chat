use core::panic;

use cosmwasm_std::Addr;
use once_cell::sync::Lazy;

use crate::prelude::*;

thread_local! {
    pub static NETWORK_CONTRACT_PAIRS:Vec<(WalletSigning, ContractKind)> = vec![
        (Wallet::neutron(), ContractKind::Server),
        (Wallet::kujira(), ContractKind::Client),
        (Wallet::stargaze(), ContractKind::Client),
         (Wallet::nois(), ContractKind::Client),
    ];
}

#[derive(Debug)]
pub struct Config {
    pub ibc_poll_delay_ms: u32,
    pub ibc_response_timeout_ms: u32,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "dev")] {
        pub static CONFIG: Lazy<Config> = Lazy::new(|| {
            Config {
                ibc_poll_delay_ms: 3000,
                // 2 minutes 
                ibc_response_timeout_ms: 60 * 2 * 1000,
            }
        });
    } else {
        pub static CONFIG: Lazy<Config> = Lazy::new(|| {
            Config {
                ibc_poll_delay_ms: 3000,
                // 2 minutes 
                ibc_response_timeout_ms: 60 * 2 * 1000,
            }
        });
    }
}
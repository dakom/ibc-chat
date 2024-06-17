use core::panic;

use awsm_web::env::{self, env_var};
use cosmwasm_std::Addr;
use once_cell::sync::Lazy;
use shared::msg::network::NetworkId;

use crate::prelude::*;

pub const WASM_ARTIFACTS_PATH:&'static str = "../wasm/artifacts";
pub const DEPLOY_CONFIG_PATH:&'static str = "..";

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
    pub query_poll_delay_ms: u32,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "dev")] {
        pub static CONFIG: Lazy<Config> = Lazy::new(|| {
            Config {
                query_poll_delay_ms: 3000,
            }
        });
    } else {
        pub static CONFIG: Lazy<Config> = Lazy::new(|| {
            Config {
                query_poll_delay_ms: 3000,
            }
        });
    }
}

pub async fn write_contract_deploy_config(contract_kind: ContractKind, env: Environment, network_id: NetworkId,  config: DeployContractConfig) -> Result<()> {
    let mut lock = DEPLOY_CONFIG.write().unwrap();

    lock.replace_contract(env, network_id, contract_kind, config);

    let s = serde_json::to_string_pretty(&*lock).map_err(|_| anyhow!("failed to serialize deploy config"))?;

    let path = file_path(&DEPLOY_CONFIG_PATH, "deploy.json");

    file_write_string(&path, &s).await.map_err(|_| anyhow!("failed to write deploy config"));

    Ok(())
}

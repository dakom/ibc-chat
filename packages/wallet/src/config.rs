use core::panic;
use std::{cell::RefCell, path::Display, pin::Pin, sync::RwLock};

use cosmwasm_std::Addr;
use futures::Future;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use shared::msg::network::NetworkId;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Environment {
    Local,
    Testnet,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Testnet => "testnet",
        }
    }
}

impl From<&str> for Environment {
    fn from(s: &str) -> Self {
        match s {
            "local" => Environment::Local,
            "testnet" => Environment::Testnet,
            _ => panic!("invalid CHAINENV, set env var to 'local' or 'testnet'"),
        }
    }
}


/****** Deploy Config ********/

// For CLI tools, we need to be able to replace this at runtime, during deploy/migration etc.
pub static DEPLOY_CONFIG: Lazy<RwLock<DeployConfig>> = Lazy::new(|| {
    let s = include_str!("../../../deploy.json");
    RwLock::new(serde_json::from_str(s).expect("failed to parse deploy.json"))
});

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeployConfig {
    pub neutron: DeployNetworkConfig,
    pub kujira: DeployNetworkConfig,
    pub stargaze: DeployNetworkConfig,
    pub nois: DeployNetworkConfig,
}

impl DeployConfig {
    pub fn contract(&self, env: Environment, network: NetworkId, contract: ContractKind) -> DeployContractConfig {
        let config = match network {
            NetworkId::Neutron => self.neutron.clone(),
            NetworkId::Kujira => self.kujira.clone(),
            NetworkId::Stargaze => self.stargaze.clone(),
            NetworkId::Nois => self.nois.clone(),
        };

        let config = match env {
            Environment::Local => config.local,
            Environment::Testnet => config.testnet,
        };

        let config = match contract {
            ContractKind::Client => config.client.expect("client contract not found"),
            ContractKind::Server => config.server.expect("server contract not found"),
        };

        config
    }

    pub fn replace_contract(&mut self, env: Environment, network: NetworkId, contract: ContractKind, config: DeployContractConfig) {
        let mut network_config = match network {
            NetworkId::Neutron => &mut self.neutron,
            NetworkId::Kujira => &mut self.kujira,
            NetworkId::Stargaze => &mut self.stargaze,
            NetworkId::Nois => &mut self.nois,
        };

        let network_config = match env {
            Environment::Local => &mut network_config.local,
            Environment::Testnet => &mut network_config.testnet,
        };

        match contract {
            ContractKind::Client => network_config.client = Some(config),
            ContractKind::Server => network_config.server = Some(config),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum ContractKind {
    Client,
    Server,
}

impl std::fmt::Display for ContractKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractKind::Client => write!(f, "client"),
            ContractKind::Server => write!(f, "server"),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeployNetworkConfig {
    pub local: DeployEnvConfig,
    pub testnet: DeployEnvConfig,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeployEnvConfig {
    pub client: Option<DeployContractConfig>,
    pub server: Option<DeployContractConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeployContractConfig {
    #[serde(rename = "codeId")]
    pub code_id: Option<u32>,
    pub address: Option<String>,
    pub hash: Option<String>,
    #[serde(rename = "ibcPort")]
    pub ibc_port: Option<String>,
}

/****** Network Config ********/

pub const NETWORK_CONFIG: Lazy<NetworkConfig> = Lazy::new(|| {
    let s = include_str!("../../../network.json");
    serde_json::from_str(s).expect("failed to parse network.json")
});

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkConfig {
    pub neutron_testnet: NetworkChainConfig,
    pub neutron_local: NetworkChainConfig,
    pub kujira_testnet: NetworkChainConfig,
    pub kujira_local: NetworkChainConfig,
    pub stargaze_testnet: NetworkChainConfig,
    pub stargaze_local: NetworkChainConfig,
    pub nois_testnet: NetworkChainConfig,
    pub nois_local: NetworkChainConfig,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkChainConfig {
    pub rpc_url: String,
    pub rest_url: String,
    pub gas_price: String,
    pub full_denom: String,
    pub denom: String,
    pub chain_id: String,
    pub addr_prefix: String,
}
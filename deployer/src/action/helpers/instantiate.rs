use cosmwasm_std::{Addr, Empty};
use crate::{bindings::crypto::HashAlgo, config::{write_contract_deploy_config, WASM_ARTIFACTS_PATH}, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstantiateKind {
    // will always re-instantiate
    Always,
    // will bail early if there's no updated config
    OnlyIfNew,
}

pub async fn instantiate(wallet: &WalletSigning, contract_kind: ContractKind, kind: InstantiateKind, config: Option<DeployContractConfig>) -> Result<Option<DeployContractConfig>> {
    let mut config = match config {
        None => {
            if kind == InstantiateKind::OnlyIfNew {
                log::info!("{} has does not have an updated hash or codeId, skipping instantiation", contract_kind);
                return Ok(None)
            } else {
                wallet.deploy_config(contract_kind)
            }
        },
        Some(config) => config
    };

    let code_id = config.code_id.expect("code_id not found");

    let resp = match contract_kind {
        ContractKind::Server => wallet.contract_instantiate(
            contract_kind.to_string(),
            code_id,
            &Empty{},
        ).await?,
        ContractKind::Client => wallet.contract_instantiate(
            contract_kind.to_string(),
            code_id,
            &ClientInstantiateMsg {
                network_id: wallet.network_id(),
            }
        ).await?,
    };

    log::info!("{} contract instantiated with address {}", contract_kind, resp.address);

    config.address = Some(resp.address);

    Ok(Some(config))
}
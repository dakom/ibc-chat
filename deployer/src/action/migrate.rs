use cosmwasm_std::{Addr, Empty};
use crate::{action::helpers::ibc_port::update_ibc_port, bindings::crypto::HashAlgo, config::{write_contract_deploy_config, NETWORK_CONTRACT_PAIRS, WASM_ARTIFACTS_PATH}, prelude::*};

use super::helpers::{
    upload::{upload, UploadKind},
    instantiate::{instantiate, InstantiateKind},
};

pub async fn run() -> Result<()> {
    let pairs = NETWORK_CONTRACT_PAIRS.with(|p| p.clone());

    for (wallet, contract_kind) in pairs {
        let config = upload(&wallet, contract_kind, UploadKind::Always).await?.context(format!("config for {} not found", contract_kind))?;

        let code_id = config.code_id.context(format!("code_id for {} not found, required for migration", contract_kind))?;
        let address = config.address.clone().context(format!("address for {} not found, required for migration", contract_kind))?;

        let _ = wallet.contract_migrate(&address, code_id, &Empty{}).await?;

        write_contract_deploy_config(contract_kind, wallet.env(), wallet.network_id(), config).await?;

        log::info!("{} contract migrated with code id {} and address {}", contract_kind, code_id, address);
    }

    Ok(())
}

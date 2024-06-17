use cosmwasm_std::{Addr, Empty};
use crate::prelude::*;

pub async fn update_ibc_port(wallet: &WalletSigning, contract_kind: ContractKind, mut config: DeployContractConfig) -> Result<DeployContractConfig> {

    let addr = config.address.clone().context(format!("contract {} address not found", contract_kind))?;

    let contract_info = match contract_kind {
        ContractKind::Client => wallet.contract_info(&addr).await?,
        ContractKind::Server => wallet.contract_info(&addr).await?,
    };

    let ibc_port = contract_info.ibc_port_id.context("ibc port not found")?;

    log::info!("updating ibc port for contract {} to {}", contract_kind, ibc_port);

    config.ibc_port = Some(ibc_port);

    Ok(config)
}
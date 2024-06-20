use cosmwasm_std::{Addr, Empty};
use crate::{action::helpers::ibc_port::update_ibc_port, bindings::crypto::HashAlgo, config::{write_contract_deploy_config, NETWORK_CONTRACT_PAIRS}, prelude::*};

use super::helpers::{
    upload::{upload, UploadKind},
    instantiate::{instantiate, InstantiateKind},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeployKind {
    // will always upload and instantiate
    AlwaysEverything,
    // will bail early if nothing changed
    OnlyIfNew,
    // will only upload if new, but will always instantiate
    AlwaysInstantiate,
}

impl From<DeployKind> for UploadKind {
    fn from(kind: DeployKind) -> Self {
        match kind {
            DeployKind::AlwaysEverything => UploadKind::Always,
            DeployKind::OnlyIfNew => UploadKind::OnlyIfNew,
            DeployKind::AlwaysInstantiate => UploadKind::OnlyIfNew,
        }
    }
}

impl From<DeployKind> for InstantiateKind {
    fn from(kind: DeployKind) -> Self {
        match kind {
            DeployKind::AlwaysEverything => InstantiateKind::Always,
            DeployKind::OnlyIfNew => InstantiateKind::OnlyIfNew,
            DeployKind::AlwaysInstantiate => InstantiateKind::Always,
        }
    }

}

pub async fn run(kind: DeployKind) -> Result<()> {
    let pairs = NETWORK_CONTRACT_PAIRS.with(|p| p.clone());

    for (wallet, contract_kind) in pairs {
        let config = upload(&wallet, contract_kind, kind.into()).await?;

        if let Some(config) = config.as_ref() {
            write_contract_deploy_config(contract_kind, wallet.env(), wallet.network_id(), config.clone()).await?;
        }

        let config = instantiate(&wallet, contract_kind, kind.into(), config).await?;

        if let Some(config) = config {
            write_contract_deploy_config(contract_kind, wallet.env(), wallet.network_id(), config.clone()).await?;
            let config = update_ibc_port(&wallet, contract_kind, config).await?;
            write_contract_deploy_config(contract_kind, wallet.env(), wallet.network_id(), config).await?;
        } else {
            log::info!("No new instantiation, skipping IBC port update too");
        } 
    }

    Ok(())
}

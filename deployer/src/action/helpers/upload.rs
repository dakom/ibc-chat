use cosmwasm_std::{Addr, Empty};
use crate::{bindings::crypto::HashAlgo, config::{write_contract_deploy_config, WASM_ARTIFACTS_PATH}, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UploadKind {
    // will always upload
    Always,
    // will bail early if contract hasn't changed 
    OnlyIfNew,
}

// if a new upload happened, returns the new config
pub async fn upload(wallet: &WalletSigning, contract_kind: ContractKind, kind: UploadKind) -> Result<Option<DeployContractConfig>> {
    let path = file_path(WASM_ARTIFACTS_PATH, &format!("{}.wasm", contract_kind));
    let data = file_read_binary(path.as_str())
        .await
        .map_err(|err| anyhow!("{:?}", err))?;

    let hash = HashAlgo::SHA256.digest(&data).await?;

    let mut config = wallet.deploy_config(contract_kind);

    let mut unchanged_hash = config.hash.as_ref() == Some(&hash);

    if kind == UploadKind::OnlyIfNew && unchanged_hash {
        if let Some(code_id) = config.code_id {
            // sanity check
            if let Ok(contract_details) = wallet.contract_code_details(code_id).await {
                if contract_details.id == code_id {
                    log::info!("{} contract already uploaded, not deploying again", contract_kind);
                    return Ok(None);
                } else {
                    return Err(anyhow!("code_id mismatch even though hash is unchanged"));
                }
            }
        }
    } 

    if unchanged_hash {
        log::info!("{} contract has not changed, but re-uploading anyway", contract_kind);
    } else {
        log::info!("{} contract has changed, re-uploading", contract_kind);
    }

    let resp = wallet.contract_upload(&data).await?;
    config.code_id = Some(resp.code_id);
    log::info!("contract uploaded with code id {}", resp.code_id);

    config.hash = Some(hash);


    Ok(Some(config))
}
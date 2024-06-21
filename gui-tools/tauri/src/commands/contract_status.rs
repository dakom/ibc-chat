use shared_gui::status::ContractStatus;
use wallet::config::{Environment, DEPLOY_CONFIG};

#[tauri::command(rename_all = "snake_case")]
pub async fn contract_status(env: Environment) -> std::result::Result<ContractStatus, String> {
    let config = DEPLOY_CONFIG.read().unwrap().clone();

    for network in config.all_networks() {
        let all_contracts = match env {
            Environment::Local => network.local.all_contracts(),
            Environment::Testnet => network.testnet.all_contracts(),
        };

        for contract in all_contracts {
            match contract {
                None => return Ok(ContractStatus::NeedsBuildAndDeploy),
                Some(contract) => {
                    match contract.gui_hash.as_ref() {
                        None => return Ok(ContractStatus::NeedsBuildAndDeploy),
                        Some(config_gui_hash) => {
                            // TODO - calculate gui hash
                        }
                    }
                }
            }
        }
    }

    Ok(ContractStatus::UpToDate)
}
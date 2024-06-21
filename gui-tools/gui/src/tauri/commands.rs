// all the commands are available via the prelude

use super::invoke;
use anyhow::Result;
use shared_gui::status::ContractStatus;
use wallet::config::Environment;

pub async fn contract_status(env: Environment) -> Result<ContractStatus> {
    #[derive(serde::Serialize)]
    struct ContractStatusArgs {
        env: Environment,
    }

    invoke("contract_status", &ContractStatusArgs { env }).await
}
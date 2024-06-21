// all the commands are available via the prelude

use super::invoke;
use anyhow::Result;
use shared_gui::status::ContractStatus;

pub async fn contract_status() -> Result<ContractStatus> {
    invoke("contract_status", &()).await
}
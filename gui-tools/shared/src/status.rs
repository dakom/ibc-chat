use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContractStatus {
    Init,
    NeedsBuildAndDeploy,
    UpToDate,
}

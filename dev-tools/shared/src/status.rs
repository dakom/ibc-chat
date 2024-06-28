use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use shared::{contract_kind::ContractKind, msg::network::NetworkId};


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContractStatusEvent {
    pub kind: ContractKind,
    pub current_deployed: HashMap<NetworkId, bool>
}

impl ContractStatusEvent {
    pub const NAME: &'static str = "contract-status";

    pub fn new(kind: ContractKind) -> Self {
        Self {
            kind,
            current_deployed: HashMap::new()
        }
    }
}

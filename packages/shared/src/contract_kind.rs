use cosmwasm_schema::cw_serde;

#[cw_serde]
#[derive(Copy, Eq)]
pub enum ContractKind {
    Client,
    Server,
}

impl std::fmt::Display for ContractKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractKind::Client => write!(f, "client"),
            ContractKind::Server => write!(f, "server"),
        }
    }
}
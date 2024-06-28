use cosmwasm_schema::cw_serde;

#[cw_serde]
#[derive(Copy, Eq, Hash)]
pub enum ContractKind {
    Client,
    Server,
}

impl ContractKind {
    pub fn all() -> &'static [Self] {
        &[ContractKind::Client, ContractKind::Server]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ContractKind::Client => "client",
            ContractKind::Server => "server",
        }
    }
}

impl std::fmt::Display for ContractKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
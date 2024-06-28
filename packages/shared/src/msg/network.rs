use anyhow::{Result, anyhow};
use cosmwasm_schema::cw_serde;

#[cw_serde]
#[derive(Eq, Copy, Hash)]
pub enum NetworkId {
    Neutron,
    Stargaze,
    Kujira,
    Nois
}

impl NetworkId {
    pub fn all() -> &'static [Self] {
        &[
            NetworkId::Neutron,
            NetworkId::Stargaze,
            NetworkId::Kujira,
            NetworkId::Nois,
        ]
    }
}

impl std::fmt::Display for NetworkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkId::Neutron => write!(f, "neutron"),
            NetworkId::Stargaze => write!(f, "stargaze"),
            NetworkId::Kujira => write!(f, "kujira"),
            NetworkId::Nois => write!(f, "nois"),
        }
    }
}

impl std::str::FromStr for NetworkId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "neutron" => Ok(NetworkId::Neutron),
            "stargaze" => Ok(NetworkId::Stargaze),
            "kujira" => Ok(NetworkId::Kujira),
            "nois" => Ok(NetworkId::Nois),
            _ => Err(anyhow!("Unknown chain id: {}", s)),
        }
    }
}


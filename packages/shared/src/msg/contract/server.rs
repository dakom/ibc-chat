use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, IbcChannel, Uint128};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get general information about the contract 
    #[returns(InfoResp)]
    Info { }
}


#[cw_serde]
pub struct InfoResp {
    pub client_channels: Vec<IbcChannel>
}
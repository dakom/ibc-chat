use std::marker::PhantomData;
use anyhow::Result;
use cosmwasm_std::{Coin, Order, Uint64};
use serde::{de::DeserializeOwned, Serialize};

use crate::{config::{ContractKind, DeployContractConfig, DEPLOY_CONFIG}, prelude::WalletSigning, response_types::*};
use shared::msg::contract::{
    client::{ChatMessagesResp, ExecuteMsg as ClientExecuteMsg, InfoResp as ClientInfoResp, QueryMsg as ClientQueryMsg},
    server::{InfoResp as ServerInfoResp, QueryMsg as ServerQueryMsg},
};

// These are just generic traits that can be implemented for any specific contract trait
// More to the point, they are the *only* traits that need to be implemented for a contract, everything else is derived
pub trait ContractAddress {
    fn address(&self) -> String;
}

pub trait ContractQuery<QueryMsg: Serialize> {
    async fn query<RESP: DeserializeOwned>(&mut self, msg: &QueryMsg) -> Result<RESP>;
}

pub trait ContractExec<ExecMsg: Serialize, ExecResponse> {
    async fn exec(&mut self, msg: &ExecMsg) -> Result<ExecResponse>;

    async fn exec_funds(&mut self, msg: &ExecMsg, funds: &[Coin]) -> Result<ExecResponse>;
}

// The specific "client" contract trait - all methods are automatically implemented on top of the generic traits
// it's still a trait, since it can be implemented for different wallet types (on-chain, multitest, etc.)
pub trait ContractClient<ExecResponse>: ContractQuery<ClientQueryMsg> + ContractExec<ClientExecuteMsg, ExecResponse> {
    async fn query_info(&mut self) -> Result<ClientInfoResp> {
        self.query(&ClientQueryMsg::Info {}).await
    }

    async fn query_chat_messages(&mut self, after_id: Option<Uint64>, order: Option<Order>) -> Result<ChatMessagesResp> {
        self.query(&ClientQueryMsg::ChatMessages { after_id, order: order.map(|order| order.into()) }).await
    }

    async fn exec_send_message(&mut self, msg: impl Into<String>) -> Result<ExecResponse> {
        self.exec(&ClientExecuteMsg::SendMessage { message: msg.into() }).await
    }
}

// The specific "server" contract trait - all methods are automatically implemented on top of the generic traits
// it's still a trait, since it can be implemented for different wallet types (on-chain, multitest, etc.)

pub trait ContractServer: ContractQuery<ServerQueryMsg> {
    async fn query_info(&mut self) -> Result<ServerInfoResp> {
        self.query(&ServerQueryMsg::Info {}).await
    }
}

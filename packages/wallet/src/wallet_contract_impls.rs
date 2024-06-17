/// The actual methods are defined in wallet::contract_traits
/// this is just the glue to make it work with WalletSigning
/// ... other glues can exist too, e.g. in multitest with a mock wallet
use crate::{bindings::{cosmjs::CosmJs, wallet_ffi::*}, config::*, contract_traits::*, response_types::*};
use cosmwasm_std::Coin;
use serde::de::DeserializeOwned;
use shared::msg::contract::{
    client::{ChatMessagesResp, ExecuteMsg as ClientExecuteMsg, InfoResp as ClientInfoResp, QueryMsg as ClientQueryMsg},
    server::{InfoResp as ServerInfoResp, QueryMsg as ServerQueryMsg},
};
use anyhow::Result;

// The specific struct for "client" contract + WalletSigning
#[derive(Clone)]
pub struct WalletSigningContractClient {
    wallet: WalletSigning,
}

// where it all gets tied together :)
impl ContractClient<TxResp> for WalletSigningContractClient {}

impl WalletSigningContractClient {
    pub fn new(wallet: WalletSigning) -> Self {
        Self { wallet }
    }
}

impl ContractAddress for WalletSigningContractClient {
    fn address(&self) -> String {
        self.wallet.deploy_config(ContractKind::Client).address.unwrap()
    }
}

impl ContractQuery<ClientQueryMsg> for WalletSigningContractClient {
    async fn query<RESP: DeserializeOwned>(&mut self, msg: &ClientQueryMsg) -> Result<RESP> {
        self.wallet.contract_query(&self.address(), msg).await
    }
}

impl ContractExec<ClientExecuteMsg, TxResp> for WalletSigningContractClient {
    async fn exec(&mut self, msg: &ClientExecuteMsg) -> Result<TxResp> {
        self.wallet.contract_exec(&self.address(), msg).await
    }

    async fn exec_funds(&mut self, msg: &ClientExecuteMsg, funds: &[Coin]) -> Result<TxResp> {
        self.wallet.contract_exec_funds(&self.address(), msg, funds).await
    }
}

// The specific struct for "server" contract + WalletSigning
#[derive(Clone)]
pub struct WalletSigningContractServer {
    wallet: WalletSigning,
}

// where it all gets tied together :)
impl ContractServer for WalletSigningContractServer {}

impl WalletSigningContractServer {
    pub fn new(wallet: WalletSigning) -> Self {
        Self { wallet }
    }
}
impl ContractAddress for WalletSigningContractServer {
    fn address(&self) -> String {
        self.wallet.deploy_config(ContractKind::Server).address.unwrap()
    }
}

impl ContractQuery<ServerQueryMsg> for WalletSigningContractServer {
    async fn query<RESP: DeserializeOwned>(&mut self, msg: &ServerQueryMsg) -> Result<RESP> {
        self.wallet.contract_query(&self.address(), msg).await
    }
}
/// The actual methods are defined in wallet::contract_traits
/// this is just the glue to make it work with the TestApp
/// ... of course, the native wallet crate has the real glue code to cosmjs for node/browser
use wallet::prelude::*;
use cosmwasm_std::{to_json_binary, Addr, Coin, CosmosMsg, WasmMsg};
use serde::de::DeserializeOwned;
use shared::msg::contract::{
    client::{ExecuteMsg as ClientExecuteMsg, QueryMsg as ClientQueryMsg},
    server::QueryMsg as ServerQueryMsg,
};
use cw_multi_test::{AppResponse, Executor};
use anyhow::Result;

use crate::app::TestApp;

// The specific struct for "client" contract + TestApp 
pub struct TestAppContractClient {
    app: TestApp,
    pub id: usize,
    pub sender: Addr,
}

// where it all gets tied together :)
impl ContractClient<AppResponse> for TestAppContractClient {}

impl TestAppContractClient{
    pub fn new(app: TestApp) -> Self {
        Self { app, id: 0, sender: Addr::unchecked("sender".to_string()) }
    }
}

impl ContractAddress for TestAppContractClient {
    fn address(&self) -> String {
        self.app.as_ref().client_contracts[self.id].to_string()
    }
}

impl ContractQuery<ClientQueryMsg> for TestAppContractClient {
    async fn query<RESP: DeserializeOwned>(&mut self, msg: &ClientQueryMsg) -> Result<RESP> {
        self.app
            .as_mut()
            .wrap()
            .query_wasm_smart(self.address(), &msg)
            .map_err(|err| err.into())
    }
}

impl ContractExec<ClientExecuteMsg, AppResponse> for TestAppContractClient {
    async fn exec(&mut self, msg: &ClientExecuteMsg) -> Result<AppResponse> {
        let cosmos_msg = CosmosMsg::Wasm(
            WasmMsg::Execute {
                contract_addr: self.address(),
                msg: to_json_binary(msg).unwrap(),
                funds: vec![],
            },
        );

        self.app.as_mut().execute(self.sender.clone(), cosmos_msg)
    }

    async fn exec_funds(&mut self, msg: &ClientExecuteMsg, funds: &[Coin]) -> Result<AppResponse> {
        let cosmos_msg = CosmosMsg::Wasm(
            WasmMsg::Execute {
                contract_addr: self.address(),
                msg: to_json_binary(msg).unwrap(),
                funds: funds.to_vec(),
            },
        );

        self.app.as_mut().execute(self.sender.clone(), cosmos_msg)
    }
}

// The specific struct for "server" contract + TestApp 

pub struct TestAppContractServer {
    app: TestApp,
    pub sender: Addr,
}

// where it all gets tied together :)
impl ContractServer for TestAppContractServer {}

impl TestAppContractServer {
    pub fn new(app: TestApp) -> Self {
        Self { app, sender: Addr::unchecked("sender".to_string()) }
    }
}

impl ContractAddress for TestAppContractServer {
    fn address(&self) -> String {
        self.app.as_ref().server_contract.to_string()
    }
}

impl ContractQuery<ServerQueryMsg> for TestAppContractServer {
    async fn query<RESP: DeserializeOwned>(&mut self, msg: &ServerQueryMsg) -> Result<RESP> {
        self.app
            .as_mut()
            .wrap()
            .query_wasm_smart(self.address(), &msg)
            .map_err(|err| err.into())
    }
}
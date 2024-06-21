use std::{collections::HashMap, ops::{Deref, DerefMut}, sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard}, vec};

use cosmwasm_std::{Addr, Empty};
use cw_multi_test::{App, ContractWrapper, Executor};
use shared::msg::{self, network::NetworkId};

use crate::multitest_contract_impls::{TestAppContractClient, TestAppContractServer};

#[derive(Clone)]
pub struct TestApp {
    inner: Arc<RwLock<TestAppInner>>,
}

impl TestApp {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(TestAppInner::new()))
        }
    }

    pub fn as_ref(&self) -> RwLockReadGuard<TestAppInner> {
        self.inner.read().unwrap()
    }

    pub fn as_mut(&self) -> RwLockWriteGuard<TestAppInner> {
        self.inner.write().unwrap()
    }

    pub fn into_contract_client(self) -> TestAppContractClient {
        TestAppContractClient::new(self)
    }

    pub fn into_contract_server(self) -> TestAppContractServer {
        TestAppContractServer::new(self)
    }
}

pub struct TestAppInner {
    #[allow(dead_code)]
    code_ids: HashMap<ContractKind, u64>,
    app: App,
    pub client_contracts: Vec<Addr>,
    pub server_contract: Addr,
}

impl TestAppInner {
    pub fn new() -> Self {
        let mut app = App::default();
        let mut code_ids = HashMap::new();

        code_ids.insert(ContractKind::Server, app.store_code(Box::new(ContractWrapper::new(
            server::entry::execute, 
            server::entry::instantiate, 
            server::entry::query
        ))));

        code_ids.insert(ContractKind::Client, app.store_code(Box::new(ContractWrapper::new(
            client::entry::execute, 
            client::entry::instantiate, 
            client::entry::query
        ))));

        let mut client_contracts = Vec::with_capacity(4);

        let network_ids = vec![
            NetworkId::Kujira,
            NetworkId::Stargaze,
            NetworkId::Nois,
            NetworkId::Neutron,
        ];

        for network_id in network_ids  {
            client_contracts.push(app.instantiate_contract(
                code_ids[&ContractKind::Client],
                Addr::unchecked("client-admin"),
                &msg::contract::client::InstantiateMsg {
                    network_id
                },
                &[],
                "client",
                None,
            ).unwrap());
        }

        let server_contract = app.instantiate_contract(
            code_ids[&ContractKind::Server],
            Addr::unchecked("server-admin"),
            &Empty {},
            &[],
            "server",
            None,
        ).unwrap();

        Self {
            app,
            code_ids,
            client_contracts,
            server_contract
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractKind {
    Client,
    Server
}

// yeah yeah, abusing this a bit :P
impl Deref for TestAppInner {
    type Target = App;
    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl DerefMut for TestAppInner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}
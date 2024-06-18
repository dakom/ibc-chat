/// Wallet is a global static
/// it's driven by bindings to cosmjs
/// which can be passed in from either node or browser
/// see bindings::cosmjs for more details

use std::cell::RefCell;

use crate::{bindings::{cosmjs::CosmJs, wallet_ffi::*}, config::*, contract_traits::*, response_types::*, wallet_contract_impls::{WalletSigningContractClient, WalletSigningContractServer}};
use base64::{prelude::BASE64_STANDARD, Engine};
use cosmwasm_std::{Addr, Coin};
use js_sys::Uint8Array;
use serde::{de::DeserializeOwned, Serialize};
use shared::{contract_kind::ContractKind, msg::network::NetworkId};
use wasm_bindgen::prelude::*;
use anyhow::{anyhow, Result};

thread_local! {
    static WALLET: Wallet = {
        Wallet::new()
    };
}
pub struct Wallet
{
    instance: RefCell<Option<WalletInstance>>,
}

impl Wallet
{
    fn new() -> Self {
        Self {
            instance: RefCell::new(None),
        }
    }

    // if seed_phrase is None, will use keplr
    //
    // this is a static function, intended to be called at some initialization point
    // it will mutably update the global static so that future calls can just call Wallet::foo()
    //
    // on the frontend, this is gated in the UI via connected_signal()
    // and in the cli tool in the start function itself
    pub async fn connect(cosmjs: CosmJs, env: Environment, seed_phrase: Option<String>) -> Result<()> {
        let res = ffi_connect(
            cosmjs,
            serde_wasm_bindgen::to_value(&*NETWORK_CONFIG).unwrap(),
            env.as_str(),
            seed_phrase,
        ).await;

        match res {
            Ok(instance) => {
                WALLET.with(|wallet| {
                    *wallet.instance.borrow_mut() = Some(instance.unchecked_into());
                    Ok(())
                })
            },
            Err(err) => Err(anyhow!("{:?}", err))
        }
    }

    pub async fn install_keplr(env: Environment) -> Result<(), JsValue> {
        log::info!("installing keplr...");
        ffi_install_keplr(
            serde_wasm_bindgen::to_value(&*NETWORK_CONFIG).unwrap(),
            env.as_str()
        ).await.map(|_| ())
    }


    pub fn get_connected() -> bool {
        WALLET.with(|wallet| {
            wallet.instance.borrow().is_some()
        })
    }

    pub fn neutron() -> WalletSigning {
        WALLET.with(|wallet| {
            wallet.instance.borrow().as_ref().unwrap().neutron()
        })
    }

    pub fn kujira() -> WalletSigning{
        WALLET.with(|wallet| {
            wallet.instance.borrow().as_ref().unwrap().kujira()
        })
    }

    pub fn stargaze() -> WalletSigning {
        WALLET.with(|wallet| {
            wallet.instance.borrow().as_ref().unwrap().stargaze()
        })
    }

    pub fn nois() -> WalletSigning {
        WALLET.with(|wallet| {
            wallet.instance.borrow().as_ref().unwrap().nois()
        })
    }

    pub fn all_clients() -> Vec<WalletSigning> {
        vec![
            Self::stargaze(),
            Self::kujira(),
            Self::nois(),
        ]
    }

    pub fn server() -> WalletSigning {
        Self::neutron()
    }

}

impl WalletSigning {
    // Contracts are app-specific
    pub fn into_contract_client(self) -> WalletSigningContractClient {
        WalletSigningContractClient::new(self)
    }

    pub fn into_contract_server(self) -> WalletSigningContractServer {
        WalletSigningContractServer::new(self)
    }

    // needed at this higher level by CLI. The WalletSigningContract* structs also use it to get the address
    // so it should always be read fresh, in case it was replaced at runtime
    pub fn deploy_config(&self, kind: ContractKind) -> DeployContractConfig {
        DEPLOY_CONFIG.read().unwrap().contract(self.env(), self.network_id(), kind)
    }

    // Generic wallet stuff 
    pub fn network_id(&self) -> NetworkId {
        self.network_id_string().parse().unwrap()
    }

    pub fn env(&self) -> Environment {
        self.env_string().as_str().into()
    }

    pub async fn balance(&self) -> Result<f64> {
        ffi_wallet_balance(&self)
            .await
            .and_then(|resp| resp.as_f64().ok_or("balance not a number".into()))
            .map_err(|err| anyhow!("{:?}", err)) 
    }


    pub async fn contract_instantiate<MSG: Serialize>(
        &self,
        label: String,
        code_id: u32,
        msg: &MSG,
    ) -> Result<ContractInstantiateResponse> {
        json_deserialize_result(ffi_contract_instantiate(self, code_id, json_serialize(msg)?, label).await)
    }

    pub async fn contract_migrate<MSG: Serialize>(
        &self,
        addr: &str,
        code_id: u32,
        msg: &MSG,
    ) -> Result<ContractMigrateResponse> {
        json_deserialize_result(ffi_contract_migrate(self, addr, code_id, json_serialize(msg)?).await)
    }

    pub async fn contract_query<MSG: Serialize, RESP: DeserializeOwned>(
        &self,
        addr: &str,
        msg: &MSG,
    ) -> Result<RESP> {
        json_deserialize_result(ffi_contract_query(self, addr, json_serialize(msg)?).await)
    }
    
    pub async fn contract_exec<MSG: Serialize>(
        &self,
        addr: &str,
        msg: &MSG,
    ) -> Result<TxResp> {
        json_deserialize_result(ffi_contract_exec(self, addr, json_serialize(msg)?).await)
    }

    pub async fn contract_exec_funds<MSG: Serialize>(
        &self,
        addr: &str,
        msg: &MSG,
        funds: &[Coin],
    ) -> Result<TxResp> {
        let funds = json_serialize(funds)?;
        let resp = json_deserialize_result(
            ffi_contract_exec_funds(self, addr, json_serialize(msg)?, funds).await,
        )?;
    
        Ok(resp)
    }

    pub async fn contract_code_details(
        &self,
        code_id: u32,
    ) -> Result<ContractCodeDetails> {
        let resp = json_deserialize_result(
            ffi_contract_code_details(self, code_id).await,
        )?;

        Ok(resp)
    }

    pub async fn contract_info(
        &self,
        addr: &str,
    ) -> Result<ContractInfo> {

        let resp = json_deserialize_result(
            ffi_contract_info(self, &addr).await,
        )?;

        Ok(resp)
    }


    pub async fn contract_upload(
        &self,
        data: &JsValue, // Buffer from NodeJS
    ) -> Result<ContractUploadResponse> {
        let resp = json_deserialize_result(
            ffi_contract_upload(self, data).await,
        )?;

        Ok(resp)
    }

    pub async fn search_tx(
        &self,
        query: &str,
    ) -> Result<Vec<IndexedTx>> {
        let resp = json_deserialize_result(
            ffi_search_tx(self, query).await,
        )?;

        Ok(resp)
    }

    pub async fn get_block(
        &self,
        height: Option<u32>,
    ) -> Result<BlockResponse> {
        let resp = json_deserialize_result(
            ffi_get_block(self, height).await,
        )?;

        Ok(resp)
    }

    pub async fn get_height(&self) -> Result<u32> {
        let resp = json_deserialize_result(
            ffi_get_height(self).await
        )?;
        Ok(resp)
    }

}


// generic helpers for any serializable msg/response
fn json_serialize(data: impl Serialize) -> Result<JsValue> {
    data.serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|err| anyhow!("{}", err))
}
fn json_deserialize<T: DeserializeOwned>(data: JsValue) -> Result<T> {
    serde_wasm_bindgen::from_value(data).map_err(|err| anyhow!("{}", err))
}
fn json_deserialize_result<T: DeserializeOwned>(result: Result<JsValue, JsValue>) -> Result<T> {
    match result {
        Ok(data) => json_deserialize(data),
        Err(err) => Err(anyhow!("{:?}", err)),
    }
}

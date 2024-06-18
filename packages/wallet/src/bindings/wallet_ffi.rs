use wasm_bindgen::prelude::*;
use super::cosmjs::CosmJs;


#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type WalletInstance;

    #[wasm_bindgen(method, getter)]
    pub fn neutron(this: &WalletInstance) -> WalletSigning;

    #[wasm_bindgen(method, getter)]
    pub fn kujira(this: &WalletInstance) -> WalletSigning;

    #[wasm_bindgen(method, getter)]
    pub fn stargaze(this: &WalletInstance) -> WalletSigning;

    #[wasm_bindgen(method, getter)]
    pub fn nois(this: &WalletInstance) -> WalletSigning;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type WalletSigning;

    #[wasm_bindgen(method, getter, js_name = "networkId")]
    pub fn network_id_string(this: &WalletSigning) -> String;

    #[wasm_bindgen(method, getter, js_name = "env")]
    pub fn env_string(this: &WalletSigning) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn signer(this: &WalletSigning) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn client(this: &WalletSigning) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn address(this: &WalletSigning) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn denom(this: &WalletSigning) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn cosmjs(this: &WalletSigning) -> CosmJs;
}

#[wasm_bindgen(module = "/src/bindings/wallet.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn ffi_connect(
        cosmjs: CosmJs,
        network_config: JsValue,
        chainenv: &str,
        // if supplied, will use direct
        // otherwise will use keplr
        seed_phrase: Option<String>,
    ) -> Result<JsValue, JsValue>;


    #[wasm_bindgen(catch)]
    pub async fn ffi_install_keplr(network_config: JsValue, chainenv: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn ffi_contract_code_details(
        wallet: &JsValue,
        code_id: u32,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn ffi_contract_info(
        wallet: &JsValue,
        addr: &str,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn ffi_contract_upload(
        wallet: &JsValue,
        data: &JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn ffi_contract_instantiate(
        wallet: &JsValue,
        code_id: u32,
        msg: JsValue,
        name: String,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn ffi_contract_migrate(
        wallet: &JsValue,
        addr: &str,
        code_id: u32,
        msg: JsValue,
    ) -> Result<JsValue, JsValue>;


    #[wasm_bindgen(catch)]
    pub async fn ffi_contract_query(
        wallet: &JsValue,
        addr: &str,
        msg: JsValue,
    ) -> Result<JsValue, JsValue>;


    #[wasm_bindgen(catch)]
    pub async fn ffi_contract_exec(
        wallet: &JsValue,
        addr: &str,
        msg: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn ffi_contract_exec_funds(
        wallet: &JsValue,
        addr: &str,
        msg: JsValue,
        funds: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn ffi_wallet_balance(
        wallet: &JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn ffi_search_tx(
        wallet: &JsValue,
        query: &str,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn ffi_get_block(
        wallet: &JsValue,
        height: Option<u32>,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn ffi_get_height(
        wallet: &JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub fn ffi_decode_tx(
        wallet: &JsValue,
        bytes: &JsValue,
    ) -> Result<JsValue, JsValue>;
}
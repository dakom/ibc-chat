use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type CosmJs;

    // these aren't actually used on the rust side, just useful for reference
    // so we know exactly what we need to export
    #[wasm_bindgen(method, getter, js_name = "DirectSecp256k1HdWallet")]
    pub fn direct_secp_wallet(this: &CosmJs) -> JsValue;

    #[wasm_bindgen(method, getter, js_name = "SigningCosmWasmClient")]
    pub fn signing_cosm_wasm_client(this: &CosmJs) -> JsValue;

    #[wasm_bindgen(method, getter, js_name = "GasPrice")]
    pub fn gas_price(this: &CosmJs) -> JsValue;

}

#[cfg(feature = "node")]
pub mod node {
    use wasm_bindgen::prelude::*;
    use super::CosmJs;

    pub fn get_cosmjs() -> CosmJs {
        let obj = js_sys::Object::new();

        js_sys::Reflect::set(&obj, &"SigningCosmWasmClient".into(), &SIGNING_COSMWASM_CLIENT).unwrap();
        js_sys::Reflect::set(&obj, &"GasPrice".into(), &GAS_PRICE).unwrap();
        js_sys::Reflect::set(&obj, &"DirectSecp256k1HdWallet".into(), &DIRECT_SECP_256_K1_HD_WALLET).unwrap();

        obj.unchecked_into()
    }

    #[wasm_bindgen(module = "@cosmjs/cosmwasm-stargate")]
    extern "C" {
        #[derive(Debug, Clone)]
        type SigningCosmWasmClient;
        #[wasm_bindgen(js_name = "SigningCosmWasmClient")]
        static SIGNING_COSMWASM_CLIENT: SigningCosmWasmClient;
    }

    #[wasm_bindgen(module = "@cosmjs/stargate")]
    extern "C" {
        #[derive(Debug, Clone)]
        type GasPrice;
        #[wasm_bindgen(js_name = "GasPrice")]
        static GAS_PRICE: GasPrice;
    }

    #[wasm_bindgen(module = "@cosmjs/proto-signing")]
    extern "C" {
        #[derive(Debug, Clone)]
        type DirectSecp256k1HdWallet;
        #[wasm_bindgen(js_name = "DirectSecp256k1HdWallet")]
        static DIRECT_SECP_256_K1_HD_WALLET: DirectSecp256k1HdWallet;
    }


}

#[cfg(feature = "web")]
pub mod web {
    use wasm_bindgen::prelude::*;

    pub fn get_cosmjs() -> super::CosmJs {
        web_sys::window().unwrap().get("CosmWasmJS").expect("window.CosmWasmJS doesn't exist").unchecked_into()
    }
}
use wasm_bindgen::prelude::*;

use crate::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern "C" {
    #[wasm_bindgen(js_name = "subtle")]
    type CryptoSubtle;
    #[wasm_bindgen(js_name = "subtle")]
    static SUBTLE: CryptoSubtle;

    #[wasm_bindgen(method, js_name = importKey, catch)]
    async fn import_key(this: &CryptoSubtle, format: &str, keyData: &Buffer, algorithm: &JsValue, extractable: bool, keyUsages: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = exportKey, catch)]
    async fn export_key(this: &CryptoSubtle, format: &str, key: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn sign(this: &CryptoSubtle, algorithm: &JsValue, key: &JsValue, data: &Buffer) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn verify(this: &CryptoSubtle, algorithm: &JsValue, key: &JsValue, signature: &Buffer, data: &Buffer) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn digest(this: &CryptoSubtle, algorithm: &JsValue, data: &Buffer) -> Result<JsValue, JsValue>;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HashAlgo {
    SHA256,
    SHA384,
    SHA512,
}

impl HashAlgo {
    fn as_str(&self) -> &str {
        match self {
            Self::SHA256 => "SHA-256",
            Self::SHA384 => "SHA-384",
            Self::SHA512 => "SHA-512",
        }
    }

    pub async fn digest(self, data: &Buffer) -> Result<String> {
        let algo = JsValue::from_str(self.as_str());
        let result = SUBTLE.digest(&algo, data).await.map_err(|err| anyhow!("{:?}", err))?;
        let result = js_sys::Uint8Array::new(&result);
        Ok(hex::encode(result.to_vec()))
    }
}

use cosmwasm_std::Event;
use serde::{Deserialize, Serialize};
use shared::tx::CosmosResponseExt;


#[derive(Deserialize, Debug, Clone)]
pub struct TxResp {
    #[serde(rename = "gasUsed")]
    pub gas_used: u64,
    #[serde(rename = "gasWanted")]
    pub gas_wanted: u64,
    #[serde(rename = "height")]
    pub height: u64,
    #[serde(rename = "transactionHash")]
    pub hash: String,
    // will always be 1 deep since we only send one message at a time
    pub logs: Option<Vec<Logs>>,
    pub events: Option<Vec<Event>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Logs {
    pub msg_index: Option<u32>,
    pub log: Option<String>,
    pub events: Vec<cosmwasm_std::Event>,
}

impl CosmosResponseExt for TxResp {
    fn events(&self) -> Box<dyn Iterator<Item = Event> + 'static> {
        if let Some(logs) = &self.logs {
            if logs.len() > 1 {
                return Box::new(
                    logs
                        .clone()
                        .into_iter()
                        .map(|log| log.events.into_iter())
                        .flatten(),
                )
            }
        }

        if let Some(events) = &self.events {
            Box::new(events.clone().into_iter())
        } else {
            Box::new(Vec::<Event>::new().into_iter())
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContractCodeDetails {
    pub id: u32,
    pub creator: String,
    pub checksum: String,
    //this also exists, but, meh, don't want to deserialize it
    //pub data: Uint8Array,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContractUploadResponse {
    /** A hex encoded sha256 checksum of the original Wasm code (that is stored on chain) */
    pub checksum: String,
    /** Size of the original wasm code in bytes */
    #[serde(rename = "originalSize")]
    pub original_size: f64,
    /** Size of the compressed wasm code in bytes */
    #[serde(rename = "compressedSize")]
    compressed_size: f64,
    /** The ID of the code asigned by the chain */
    #[serde(rename = "codeId")]
    pub code_id: u32,
    #[serde(rename = "gasUsed")]
    pub gas_used: u64,
    #[serde(rename = "gasWanted")]
    pub gas_wanted: u64,
    pub height: u64,
    #[serde(rename = "transactionHash")]
    pub hash: String,
    /** @deprecated Not filled in Cosmos SDK >= 0.50. Use events instead. */
    pub logs: Option<Vec<Logs>>,
    pub events: Option<Vec<Event>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContractInstantiateResponse{
    #[serde(rename = "contractAddress")]
    pub address: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: u64,
    #[serde(rename = "gasWanted")]
    pub gas_wanted: u64,
    pub height: u64,
    #[serde(rename = "transactionHash")]
    pub hash: String,
    /** @deprecated Not filled in Cosmos SDK >= 0.50. Use events instead. */
    pub logs: Option<Vec<Logs>>,
    pub events: Option<Vec<Event>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContractInfo {
    pub address: String,
    #[serde(rename = "codeId")]
    pub code_id: u32,
    pub creator: String,
    pub admin: Option<String>,
    pub label: String,
    #[serde(rename = "ibcPortId")]
    pub ibc_port_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContractMigrateResponse {
    #[serde(rename = "gasUsed")]
    pub gas_used: u64,
    #[serde(rename = "gasWanted")]
    pub gas_wanted: u64,
    pub height: u64,
    #[serde(rename = "transactionHash")]
    pub hash: String,
    /** @deprecated Not filled in Cosmos SDK >= 0.50. Use events instead. */
    pub logs: Option<Vec<Logs>>,
    pub events: Option<Vec<Event>>
}
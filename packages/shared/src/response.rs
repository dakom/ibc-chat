use std::collections::HashMap;

use anyhow::Result;
use cosmwasm_std::{
    from_json, to_json_binary, wasm_execute, Binary, CosmosMsg, Empty, Event, IbcBasicResponse, IbcReceiveResponse, QueryResponse, Response, StdAck, SubMsg, WasmMsg
};
use cw2::ContractVersion;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::ibc::IbcAck;

/// Helper data type, following builder pattern, for constructing a [Response].
pub struct ResponseBuilder {
    resp: Response,
    event_type: EventType,
    event_type_count: HashMap<String, u32>,
}

enum EventType {
    MuteEvents,
    EmitEvents {
        common_attrs: Vec<(&'static str, String)>,
    },
}

fn standard_event_attributes(
    ContractVersion { contract, version }: ContractVersion,
) -> Vec<(&'static str, String)> {
    vec![
        ("contract_version", version),
        ("contract_kind", contract),
    ]
}

impl ResponseBuilder {
    /// Initialize a new builder.
    pub fn new(contract_version: ContractVersion) -> Self {
        ResponseBuilder {
            resp: Response::new(),
            event_type: EventType::EmitEvents {
                common_attrs: standard_event_attributes(contract_version),
            },
            event_type_count: HashMap::new(),
        }
    }

    /// Create a response where the event methods are no-ops.
    pub fn new_mute_events() -> Self {
        ResponseBuilder {
            resp: Response::new(),
            event_type: EventType::MuteEvents,
            event_type_count: HashMap::new(),
        }
    }

    /// Finalize the builder and generate the final response.
    pub fn into_response(self) -> Response {
        self.resp
    }

    /// Add a new [CosmosMsg] to the response.
    pub fn add_message(&mut self, msg: impl Into<CosmosMsg<Empty>>) {
        self.resp.messages.push(SubMsg::new(msg.into()));
    }

    /// Add a submessage for instantiating a new contract.
    pub fn add_instantiate_submessage<
        I: Into<u64>,
        A: Into<String>,
        L: Into<String>,
        T: Serialize,
    >(
        &mut self,
        id: I,
        admin: A,
        code_id: u64,
        label: L,
        msg: &T,
    ) -> Result<()> {
        let payload = to_json_binary(msg)?;

        // the common case
        // more fine-grained control via raw submessage
        let msg = WasmMsg::Instantiate {
            admin: Some(admin.into()),
            code_id,
            msg: payload,
            funds: vec![],
            label: label.into(),
        };
        self.add_raw_submessage(
            // the common case
            // more fine-grained control via raw submessage
            SubMsg::reply_on_success(msg, id.into()),
        );

        Ok(())
    }

    /// Add a new one-shot submessage execution.
    pub fn add_execute_submessage_oneshot<C: Into<String>, T: Serialize>(
        &mut self,
        contract: C,
        msg: &T,
    ) -> Result<()> {
        self.add_raw_submessage(
            // the common case
            // more fine-grained control via raw submessage
            SubMsg::new(wasm_execute(
                contract,
                msg,
                // the common case, no coins
                vec![],
            )?),
        );

        Ok(())
    }

    /// Add a raw submsg. Helpful if you need to handle a reply.
    pub fn add_raw_submessage(&mut self, msg: SubMsg<Empty>) {
        self.resp.messages.push(msg);
    }

    /// Add an event to the response.
    pub fn add_event(&mut self, event: impl Into<Event>) {
        let event: Event = event.into();

        match &self.event_type {
            EventType::MuteEvents => (),
            EventType::EmitEvents { common_attrs } => {
                let mut event = event.add_attributes(common_attrs.clone());

                let event_type_count = self.event_type_count.entry(event.ty.clone()).or_default();

                if *event_type_count > 0 {
                    event.ty = format!("{}-{}", event.ty, *event_type_count);
                }

                *event_type_count += 1;

                self.resp.events.push(event)
            }
        }
    }

    /// Set response data
    pub fn set_data(&mut self, data: &impl Serialize) -> Result<()> {
        match self.resp.data {
            None => {
                let data = to_json_binary(data)?;
                self.resp.data = Some(data);
            }
            Some(_) => anyhow::bail!("data already exists, use update_data instead"),
        }

        Ok(())
    }

    /// Get response data
    pub fn get_data<T: DeserializeOwned>(&self) -> Result<Option<T>> {
        match &self.resp.data {
            None => Ok(None),
            Some(data) => Ok(Some(from_json(data)?)),
        }
    }

    /// Remove response data
    pub fn remove_data(&mut self) {
        self.resp.data = None;
    }

    /// Update response data
    pub fn update_data<T: Serialize + DeserializeOwned>(
        &mut self,
        f: impl FnOnce(Option<T>) -> T,
    ) -> Result<()> {
        let data = self.get_data()?;
        let updated = f(data);
        self.resp.data = Some(to_json_binary(&updated)?);

        Ok(())
    }

    /// Turn the accumulated response into an IBC Basic response
    pub fn into_ibc_response(self) -> IbcBasicResponse {
        let mut resp = IbcBasicResponse::default();
        resp.messages = self.resp.messages;
        resp.attributes = self.resp.attributes;
        resp.events = self.resp.events;

        resp
    }

    /// Turn the accumulated response into an IBC Receive success response
    pub fn into_ibc_recv_response_success(self, data: Option<Binary>) -> IbcReceiveResponse {
        let mut resp = IbcReceiveResponse::new(StdAck::success(data.unwrap_or(b"\x01".into())));
        resp.messages = self.resp.messages;
        resp.attributes = self.resp.attributes;
        resp.events = self.resp.events;

        resp
    }

    /// Turn the accumulated response into an IBC Receive fail response
    pub fn into_ibc_recv_response_fail(self, error: anyhow::Error) -> IbcReceiveResponse {
        let mut resp = IbcReceiveResponse::new(StdAck::error(error.to_string()));
        resp.messages = self.resp.messages;
        resp.attributes = self.resp.attributes;
        resp.events = self.resp.events;

        resp
    }
}

/// Makes it easy to call .query_result() on any Serialize
/// and standardizes so query() entry points also return a ContractResult
pub trait QueryResponseExt {
    /// Convert the value to its JSON representation
    fn query_result(&self) -> Result<QueryResponse>;
}
impl<T: Serialize> QueryResponseExt for T {
    fn query_result(&self) -> Result<QueryResponse> {
        to_json_binary(self).map_err(|err| err.into())
    }
}
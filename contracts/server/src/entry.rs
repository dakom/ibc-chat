use cosmwasm_std::{
    entry_point, Deps, DepsMut, Empty, Env, IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcChannelOpenResponse, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, MessageInfo, QueryResponse, Response
};
use cw2::{get_contract_version, set_contract_version};
use shared::{
    msg::contract::server::{InfoResp, QueryMsg}, response::{QueryResponseExt, ResponseBuilder},
};
use anyhow::Result;

use crate::state::{State, StateContext};
// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty) -> Result<Response> {
    let (state, mut ctx) = StateContext::new(deps, env)?;

    Ok(ctx.response.into_response())
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse> {
    let (state, store) = State::new(deps, env)?;

    match msg {
        QueryMsg::Info {  } => {
            let client_channels = state.get_client_channels(store)?;
            let info = InfoResp {
                client_channels
            };
            info.query_result()
        }
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, env: Env, msg: Empty) -> Result<Response> {
    let (state, mut ctx) = StateContext::new(deps, env)?;

    Ok(ctx.response.into_response())
}

/// Handles the `OpenInit` and `OpenTry` parts of the IBC handshake.
#[entry_point]
pub fn ibc_channel_open(
    deps: DepsMut,
    env: Env,
    msg: IbcChannelOpenMsg,
) -> Result<IbcChannelOpenResponse> {
    let (state, _) = StateContext::new(deps, env)?;
    state.handle_ibc_channel_open(msg)?;
    Ok(None)
}

#[entry_point]
pub fn ibc_channel_connect(
    deps: DepsMut,
    env: Env,
    msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse> {
    let (state, mut ctx) = StateContext::new(deps, env)?;
    state.handle_ibc_channel_connect(&mut ctx, msg)?;
    Ok(ctx.response.into_ibc_response())
}

#[entry_point]
pub fn ibc_channel_close(
    deps: DepsMut,
    env: Env,
    msg: IbcChannelCloseMsg,
) -> Result<IbcBasicResponse> {
    let (state, mut ctx) = StateContext::new(deps, env)?;
    state.handle_ibc_channel_close(&mut ctx, msg)?;
    Ok(ctx.response.into_ibc_response())
}

#[entry_point]
pub fn ibc_packet_receive(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse> {
    let (state, mut ctx) = StateContext::new(deps, env)?;
    state.handle_ibc_packet_receive(&mut ctx, msg)?;

    Ok(ctx.response.into_ibc_recv_response_success(None))
}

#[entry_point]
pub fn ibc_packet_ack(deps: DepsMut, env: Env, ack: IbcPacketAckMsg) -> Result<IbcBasicResponse> {
    let (state, ctx) = StateContext::new(deps, env)?;
    state.handle_ibc_packet_ack(ack)?;
    Ok(ctx.response.into_ibc_response())
}

#[entry_point]
pub fn ibc_packet_timeout(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse> {
    let (state, ctx) = StateContext::new(deps, env)?;
    state.handle_ibc_packet_timeout(msg)?;
    Ok(ctx.response.into_ibc_response())
}

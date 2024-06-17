use cosmwasm_std::{
    entry_point, to_json_binary, Deps, DepsMut, Empty, Env, IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcChannelOpenResponse, IbcMsg, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, IbcTimeout, MessageInfo, QueryResponse, Response
};
use cw2::{get_contract_version, set_contract_version};
use shared::{
    ibc::TIMEOUT_SECONDS, msg::{contract::client::{ChatMessage, ChatMessagesResp, ExecuteMsg, InfoResp, InstantiateMsg, QueryMsg}, ibc::IbcExecuteMsg}, response::{QueryResponseExt, ResponseBuilder}
};
use anyhow::{Context, Result};

use crate::state::{State, StateContext};
// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let (state, mut ctx) = StateContext::new(deps, env)?;
    state.set_network_id(&mut ctx, msg.network_id)?;



    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response> {
    let (state, mut ctx) = StateContext::new(deps, env)?;

    match msg {
        ExecuteMsg::SendMessage { message } => {
            let network_id = state.get_network_id(ctx.store)?;
            let message = ChatMessage {
                user: info.sender.clone(),
                network_id,
                message: message.clone(),
            };
            // First we store the message in our local state
            state.push_chat_message(&mut ctx, message.clone())?;

            // Then we send it to the server for broadcasting
            let msg = IbcExecuteMsg::SendMessageToServer { 
                message
            };
    
            // outbound IBC message, where packet is then received on other chain
            let channel_id = state
                .get_server_channel(ctx.store)?
                .context("server channel not set")?
                .endpoint
                .channel_id;
    
            ctx.response_mut().add_message(IbcMsg::SendPacket {
                channel_id,
                data: to_json_binary(&msg)?,
                timeout: IbcTimeout::with_timestamp(state.env.block.time.plus_seconds(TIMEOUT_SECONDS)),
            });

        }
    }

    Ok(ctx.response.into_response())
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse> {
    let (state, store) = State::new(deps, env)?;

    match msg {
        QueryMsg::Info {  } => {
            let server_channel = state.get_server_channel(store)?;
            let info = InfoResp {
                server_channel,
                network_id: state.get_network_id(store)?,
            };
            info.query_result()
        },
        QueryMsg::ChatMessages { after_id, order } => {
            let messages = state.get_chat_messages(store, after_id, order.map(|order| order.into()))?;
            ChatMessagesResp {
                messages
            }.query_result()
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

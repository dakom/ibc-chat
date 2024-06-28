// all the commands are available via the prelude

use super::{invoke, invoke_no_args_no_resp, invoke_no_resp, listen};
use anyhow::Result;
use shared_dev_tools::{process::{ProcessId, ProcessKind}, status::ContractStatusEvent};
use wallet::config::Environment;
use wasm_bindgen::prelude::*;

pub async fn get_contract_status(env: Environment) -> Result<Vec<ContractStatusEvent>> {
    #[derive(serde::Serialize)]
    struct ContractStatusArgs {
        env: Environment,
    }

    invoke("get_contract_status", &ContractStatusArgs { env }).await

}

pub async fn start_contract_status_watcher(env: Environment, on_event: impl Fn(ContractStatusEvent) + 'static) -> Result<Closure<dyn FnMut(JsValue)>> {
    #[derive(serde::Serialize)]
    struct ContractStatusArgs {
        env: Environment,
    }

    let callback = listen(ContractStatusEvent::NAME, on_event).await?; 

    invoke_no_resp("start_contract_status_watcher", &ContractStatusArgs { env }).await;

    Ok(callback)
}

pub async fn stop_contract_status_watcher() -> Result<()> {
    invoke_no_args_no_resp("stop_contract_status_watcher").await
}


pub async fn start_process(kind: ProcessKind, env: Environment) -> Result<ProcessId> {
    #[derive(serde::Serialize)]
    struct StartProcessArgs {
        kind: ProcessKind,
        env: Environment,
    }

    invoke("start_process", &StartProcessArgs{ kind, env }).await
}

pub async fn kill_process(process_id: ProcessId) -> Result<()> {
    #[derive(serde::Serialize)]
    struct KillProcessArgs {
        process_id: ProcessId,
    }

    invoke_no_resp("kill_process", &KillProcessArgs{ process_id }).await
}

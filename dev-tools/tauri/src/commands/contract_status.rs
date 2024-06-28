use std::sync::{Arc, Mutex};

use processor::monitor::{watch_contracts_deployed, ContractMonitor};
use shared_dev_tools::status::ContractStatusEvent;
use tauri::{async_runtime::{spawn, JoinHandle}, AppHandle, Manager, State};
use wallet::config::Environment;
use crate::config::CONFIG;
use anyhow::anyhow;

// TODO - return the event
#[tauri::command(rename_all = "snake_case")]
pub async fn get_contract_status(env: Environment) -> std::result::Result<Vec<ContractStatusEvent>, String> {
    async fn inner(env: Environment) -> anyhow::Result<Vec<ContractStatusEvent>> {
        let monitors = ContractMonitor::try_all(&CONFIG.contracts_dir)?;
        let mut res = Vec::with_capacity(monitors.len());

        for monitor in monitors {
            let kind = monitor.kind;
            let current_deployed = monitor.current_deployed_networks(env).await?;

            res.push(ContractStatusEvent {
                kind,
                current_deployed
            });
        }
        Ok(res)
    }

    inner(env).await.map_err(|e| e.to_string())
}

#[derive(Default)]
pub struct StatusWatcherHandle(pub Mutex<Option<JoinHandle<()>>>);

// This will start a watcher that will emit events when the contract status changes
// since it is spawned off into the background and runs "forever", the handle is set in the state
// and stop_contract_status_watcher can be used to stop it
// if it's called without calling stop_contract_status_watcher, then the old watcher will be stopped
#[tauri::command(rename_all = "snake_case")]
pub fn start_contract_status_watcher(app: AppHandle, state: State<StatusWatcherHandle>, env: Environment) -> std::result::Result<(), String> {

    fn inner(app: AppHandle, state: State<StatusWatcherHandle>, env: Environment) -> anyhow::Result<()> {

        let drop_logger = Arc::new(DropLogger{});

        let monitors = ContractMonitor::try_all(&CONFIG.contracts_dir)?;
        let future_watcher = watch_contracts_deployed(env, monitors, move |(event, _notify_event)| {
            let app = app.clone();
            let drop_logger = drop_logger.clone();
            async move {
                let _ = drop_logger;
                app.emit(ContractStatusEvent::NAME, event)?; 
                Ok(())
            }
        });

        let handle = spawn(async move {
            future_watcher.await.unwrap();
        });

        let mut lock = state.inner().0.lock().map_err(|err| anyhow!("{:?}", err))?;
        *lock = Some(handle);

        Ok(())
    }

    inner_stop_contract_status_watcher(&state).map_err(|e| e.to_string())?;
    inner(app, state, env).map_err(|e| e.to_string())

}

#[tauri::command(rename_all = "snake_case")]
pub fn stop_contract_status_watcher(state: State<StatusWatcherHandle>) -> std::result::Result<(), String> {
    inner_stop_contract_status_watcher(&state).map_err(|e| e.to_string())
}

fn inner_stop_contract_status_watcher(state: &State<StatusWatcherHandle>) -> anyhow::Result<()> {
    let mut lock = state.inner().0.lock().map_err(|e| anyhow!("{:?}", e))?;
    if let Some(handle) = lock.take() {
        handle.abort();
    } 

    Ok(())
}

struct DropLogger {
}

impl Drop for DropLogger {
    fn drop(&mut self) {
        println!("Contract watcher dropped");
    }
}
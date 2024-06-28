mod sources;
mod hash;
mod watcher;

use std::{collections::HashMap, future::Future, path::{Path, PathBuf}, sync::Arc};
use futures::{stream::FuturesUnordered, StreamExt};
use hash::hash_files;
use shared::{contract_kind::ContractKind, msg::network::NetworkId};
use anyhow::{Result, anyhow};
use shared_dev_tools::status::ContractStatusEvent;
use sources::get_source_files;
use wallet::config::{DeployContractConfig, Environment, DEPLOY_CONFIG};
use watcher::watch_files;


pub struct ContractMonitor {
    pub kind: ContractKind,
    pub paths: Vec<PathBuf> 
}

impl ContractMonitor {
    pub fn try_all(parent_dir: impl AsRef<Path>) -> Result<Vec<Self>> {
        ContractKind::all()
            .into_iter()
            .map(|kind| Self::try_new(*kind, parent_dir.as_ref()))
            .collect()
    }

    pub fn try_new(kind: ContractKind, parent_dir: impl AsRef<Path>) -> Result<Self> {

        let base_path = parent_dir.as_ref().join(kind.as_str());
        let mut paths = get_source_files(&base_path)?;

        paths.sort();

        Ok(Self { kind, paths })
    }

    pub async fn current_deployed(&self, env: Environment, network_id: NetworkId) -> Result<bool> {
        let config = self.config(env, network_id)?;
        match config.src_hash {
            Some(hash) => {
                let current_hash = self.sources_hash().await?;
                Ok(hash == current_hash)
            },
            None => Ok(false)
        }
    }

    pub async fn current_deployed_networks(&self, env: Environment) -> Result<HashMap<NetworkId, bool>> {
        let mut results = HashMap::new();

        for network_id in NetworkId::all() {
            let network_id = *network_id;

            // THIS IS PROPRIETARY TO THE APP!
            let should_check = if network_id == NetworkId::Neutron {
                self.kind == ContractKind::Server
            } else {
                self.kind != ContractKind::Server
            };

            if should_check {
                results.insert(network_id, self.current_deployed(env, network_id).await?);
            }
        }

        Ok(results)

    }

    pub async fn watch_deployed<F, A>(&self, env: Environment, on_change: F) -> Result<()> 
    where 
        F: Fn((ContractStatusEvent, notify::Event)) -> A,
        A: Future<Output = Result<()>>,
    {

        watch_files(self.paths.clone(), |notify_event| {
            async {
                on_change((ContractStatusEvent{
                    kind: self.kind, 
                    current_deployed: self.current_deployed_networks(env).await?
                }, notify_event)).await?;

                Ok(())
            }
        }).await 
    }

    pub async fn sources_hash(&self) -> Result<String> {
        hash_files(&self.paths).await
    }

    pub fn config(&self, env: Environment, network_id: NetworkId) -> Result<DeployContractConfig> {
        Ok(DEPLOY_CONFIG.read().map_err(|err| anyhow!("{:?}", err))?.contract(env, network_id, self.kind))
    }

}

pub async fn watch_contracts_deployed<F, A>(env: Environment, monitors: impl IntoIterator<Item = ContractMonitor>, on_change: F) -> Result<()> 
where 
    F: Fn((ContractStatusEvent, notify::Event)) -> A,
    A: Future<Output = Result<()>>
{
    let on_change = Arc::new(on_change);

    let mut futures:FuturesUnordered<_> = monitors.into_iter().map(move |monitor| {
        let on_change = on_change.clone();
        async move {
            monitor.watch_deployed(env, move |event| {
                let on_change = on_change.clone();
                async move {
                    on_change(event).await
                }
            }).await
        }
    }).collect();

    while let Some(res) = futures.next().await { 
        res?;
    }

    Ok(())
}
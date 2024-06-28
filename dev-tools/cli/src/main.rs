mod args;

use anyhow::Result;
use args::{Args, Command};
use clap::Parser;
use processor::monitor::{watch_contracts_deployed, ContractMonitor};

#[tokio::main]
async fn main() -> Result<()> {

    let cli_args = Args::parse();

    match cli_args.command {
        Command::ShowContractsHash {} => {
            let monitors = ContractMonitor::try_all(cli_args.contracts_dir)?;

            for monitor in monitors {
                let current_deployed = monitor.current_deployed_networks(cli_args.env).await?;
                println!("{}: {:#?}", monitor.kind, current_deployed);
            }
        },

        Command::WatchContracts {  } => {
            let monitors = ContractMonitor::try_all(cli_args.contracts_dir)?;
            watch_contracts_deployed(cli_args.env, monitors, |event_tuple| {
                async move {
                    println!("{:#?}", event_tuple);
                    Ok(())
                }
            }).await?;
        }
    }

    Ok(())
}
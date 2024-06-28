use std::path::PathBuf;

use clap::{Parser, Subcommand};
use wallet::config::Environment;

// https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value = "../../contracts")]
    pub contracts_dir: PathBuf,

    #[command(subcommand)]
    pub command: Command,

    #[clap(short, long, default_value_t = Environment::Local)]
    pub env: Environment,
}

#[derive(Subcommand)]
pub enum Command {
    ShowContractsHash {
    },
    WatchContracts {
    },
}
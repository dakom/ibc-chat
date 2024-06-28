use std::path::PathBuf;

use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config {
        contracts_dir: PathBuf::from("../../contracts"),
    }
});

#[derive(Debug)]
pub struct Config {
    pub contracts_dir: PathBuf 
}

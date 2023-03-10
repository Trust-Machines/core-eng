use clap::Parser;
use serde::Deserialize;
use std::fs;
use toml;

#[derive(Clone, Deserialize, Default, Debug)]
pub struct Config {
    pub http_relay_url: String,
    pub total_signers: usize,
    pub total_keys: usize,
    pub keys_threshold: usize,
    pub max_party_id: usize,
    pub frost_state_file: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Config file path
    #[arg(short, long)]
    pub config: String,

    /// Start a signing round
    #[arg(short, long)]
    pub start: bool,

    /// ID associated with signer
    #[arg(short, long)]
    pub id: u32,
}

impl Config {
    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Config, String> {
        let content = fs::read_to_string(path).map_err(|e| format!("Invalid path: {}", &e))?;
        toml::from_str(&content).map_err(|e| format!("Invalid toml: {}", e))
    }
}

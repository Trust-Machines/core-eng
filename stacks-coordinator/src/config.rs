// TODO: Set appropriate types
type ContractIdentifier = String;
type StacksPrivateKey = String;
type BitcoinPrivateKey = String;
type Url = String;

#[derive(serde::Deserialize)]
pub struct Config {
    pub sbtc_contract: ContractIdentifier,
    pub stacks_private_key: StacksPrivateKey,
    pub bitcoin_private_key: BitcoinPrivateKey,
    pub signer_relay_url: String,
    pub stacks_node_rpc_url: Url,
    pub bitcoin_node_rpc_url: Url,
    pub frost_dkg_round_id: u64,
}

impl Config {
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self> {
        Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to read file: {0}")]
    FileReadingError(#[from] std::io::Error),

    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] toml::de::Error),
}

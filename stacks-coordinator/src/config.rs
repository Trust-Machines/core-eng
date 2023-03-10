use crate::error::Result;
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
    pub stacks_node_rpc_url: Url,
    pub bitcoin_node_rpc_url: Url,
    pub frost_dkg_round_id: u64,
    pub signer_config_path: String,
}

impl Config {
    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Self> {
        Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
    }
}

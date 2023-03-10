pub mod coordinator;

use coordinator::Coordinator;
use frost_signer::{
    config::Config,
    net::{HttpNet, HttpNetListen},
};

pub const DEVNET_COORDINATOR_ID: usize = 0;
pub const DEVNET_COORDINATOR_DKG_ID: u64 = 0; //TODO: Remove, this is a correlation id

pub fn create_coordinator(
    path: impl AsRef<std::path::Path>,
) -> Result<Coordinator<HttpNetListen>, String> {
    let config = Config::from_path(path)?;

    let net: HttpNet = HttpNet::new(config.http_relay_url.clone());
    let net_listen: HttpNetListen = HttpNetListen::new(net, vec![]);

    Ok(Coordinator::new(
        DEVNET_COORDINATOR_ID,
        DEVNET_COORDINATOR_DKG_ID,
        &config,
        net_listen,
    ))
}

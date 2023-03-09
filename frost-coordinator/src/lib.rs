pub mod coordinator;

use coordinator::Coordinator;
use frost_signer::{
    config::Config,
    net::{HttpNet, HttpNetListen},
};

pub const DEVNET_COORDINATOR_ID: usize = 0;
pub const DEVNET_COORDINATOR_DKG_ID: u64 = 0; //TODO: Remove, this is a correlation id

pub fn create_coordinator() -> Coordinator<HttpNetListen> {
    let config = Config::from_path("conf/stacker.toml").unwrap();

    let net: HttpNet = HttpNet::new(config.stacks_node_url.clone());
    let net_listen: HttpNetListen = HttpNetListen::new(net, vec![]);

    Coordinator::new(
        DEVNET_COORDINATOR_ID,
        DEVNET_COORDINATOR_DKG_ID,
        &config,
        net_listen,
    )
}

use clap::Parser;

use frost_signer::config::Config;
use frost_signer::logging;
use frost_signer::net::{HttpNet, HttpNetListen};

use frost_coordinator::coordinator::{Command, Coordinator};

const DEVNET_COORDINATOR_ID: usize = 0;
const DEVNET_COORDINATOR_DKG_ID: u64 = 0; //TODO: Remove, this is a correlation id

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

fn main() {
    logging::initiate_tracing_subscriber(tracing::Level::INFO).unwrap();

    let cli = Cli::parse();
    let config = Config::from_file("conf/stacker.toml").unwrap();

    let net: HttpNet = HttpNet::new(config.common.stacks_node_url.clone());
    let net_listen: HttpNetListen = HttpNetListen::new(net, vec![]);
    let mut coordinator = Coordinator::new(
        DEVNET_COORDINATOR_ID,
        DEVNET_COORDINATOR_DKG_ID,
        config.common.total_signers,
        config.common.total_parties,
        config.common.minimum_parties,
        net_listen,
    );

    coordinator
        .run(&cli.command)
        .expect("Failed to execute command");
}

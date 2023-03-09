use clap::Parser;

use frost_coordinator::coordinator::Command;
use frost_coordinator::create_coordinator;
use frost_signer::logging;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

fn main() {
    logging::initiate_tracing_subscriber(tracing::Level::INFO).unwrap();

    let cli = Cli::parse();
    // let config = Config::from_path("conf/stacker.toml").unwrap();
    // let net: HttpNet = HttpNet::new(config.stacks_node_url.clone());
    // let net_listen: HttpNetListen = HttpNetListen::new(net, vec![]);

    // let mut coordinator = Coordinator::new(
    //     DEVNET_COORDINATOR_ID,
    //     DEVNET_COORDINATOR_DKG_ID,
    //     &config,
    //     net_listen,
    // );
    let mut coordinator = create_coordinator();

    coordinator
        .run(&cli.command)
        .expect("Failed to execute command");
}

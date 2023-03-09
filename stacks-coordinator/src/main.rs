use clap::Parser;
use frost_coordinator::create_coordinator;
use frost_signer::logging;
use stacks_coordinator::cli::{Cli, Command};
use stacks_coordinator::config::Config;
use stacks_coordinator::frost_coordinator::FrostCoordinator;

fn main() {
    let cli = Cli::parse();

    // Initialize logging
    logging::initiate_tracing_subscriber(if cli.debug {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    })
    .unwrap();

    //TODO: get config from sBTC contract
    let config = Config::from_path("../conf/coordinator.toml".to_string()).unwrap();

    // Determine what action the caller wishes to perform
    match cli.command {
        Command::Run => {
            println!("Running coordinator");
        }
        Command::Dkg => {
            println!("Running DKG");
            let mut coordinator = create_coordinator();
            coordinator.run_dkg_round();
        }
    };
}

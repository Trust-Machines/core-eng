use clap::Parser;
use frost_coordinator::create_coordinator;
use frost_signer::logging;
use stacks_coordinator::cli::{Cli, Command};
use stacks_coordinator::config::Config;
use stacks_coordinator::coordinator::StacksCoordinator;
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

    //TODO: get configs from sBTC contract
    let config = Config::from_path("conf/coordinator.toml".to_string()).unwrap();
    let mut stacks_coordinator = StacksCoordinator::from(config);
    // Determine what action the caller wishes to perform
    match cli.command {
        Command::Run => {
            println!("Running coordinator");
            //TODO: set up coordination with the stacks node
            //stacks_coordinator.run();
        }
        Command::Dkg => {
            println!("Running DKG Round");
            stacks_coordinator.run_dkg_round();
        }
    };
}

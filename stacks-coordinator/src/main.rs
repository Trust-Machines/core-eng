use clap::Parser;
use frost_signer::logging;
use stacks_coordinator::cli::{Cli, Command};
use stacks_coordinator::config::Config;
use stacks_coordinator::coordinator::StacksCoordinator;
use tracing::warn;

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
    match Config::from_path(&cli.config) {
        Ok(mut config) => {
            config.signer_config_path = cli.signer_config;
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
                    if let Err(e) = stacks_coordinator.run_dkg_round() {
                        warn!("DKG found encountered an error: {}", e);
                    }
                }
            };
        }
        Err(e) => {
            warn!("An error occrred reading config file {}: {}", cli.config, e);
        }
    }
}

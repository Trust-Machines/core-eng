use clap::Parser;

use frost_coordinator::coordinator::Command;
use frost_coordinator::create_coordinator;
use frost_signer::logging;
use tracing::warn;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Config file path
    #[arg(short, long)]
    config: String,
    /// Subcommand action to take
    #[command(subcommand)]
    pub command: Command,
}

fn main() {
    logging::initiate_tracing_subscriber(tracing::Level::INFO).unwrap();

    let cli = Cli::parse();
    match create_coordinator(cli.config) {
        Ok(mut coordinator) => {
            let result = coordinator.run(&cli.command);
            if let Err(e) = result {
                warn!("Failed to execute command: {}", e);
            }
        }
        Err(e) => {
            warn!("Failed to create coordinator: {}", e);
        }
    }
}

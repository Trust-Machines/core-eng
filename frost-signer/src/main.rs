use clap::Parser;
use tracing::{info, warn};

use frost_signer::config::{Cli, Config};
use frost_signer::logging;
use frost_signer::signer::Signer;

fn main() {
    logging::initiate_tracing_subscriber(tracing::Level::INFO).unwrap();

    let cli = Cli::parse();

    match Config::from_path(cli.config.clone()) {
        Ok(config) => {
            let mut signer = Signer::new(config, cli.id);
            info!("{} signer id #{}", frost_signer::version(), signer.frost_id); // sign-on message

            //Start listening for p2p messages
            if let Err(e) = signer.start_p2p_sync() {
                warn!("An error occurred in the P2P Network: {}", e);
            }
        }
        Err(e) => {
            warn!("An error occrred reading config file {}: {}", cli.config, e);
        }
    }
}

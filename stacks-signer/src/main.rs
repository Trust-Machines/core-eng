use clap::{Parser, Subcommand};
use frost_signer::logging;
use stacks_signer::secp256k1::Secp256k1;

///Command line interface for stacks signer
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    debug: bool,

    /// Subcommand action to take
    #[clap(subcommand)]
    action: Action,
}

/// Possible actions that stacks signer can perform
#[derive(Subcommand)]
enum Action {
    /// Generate Secp256k1 Private Key
    Secp256k1(Secp256k1),
}

fn main() {
    let cli = Cli::parse();

    // Initialize logging
    logging::initiate_tracing_subscriber(if cli.debug {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    })
    .unwrap();

    // Determine what action the caller wishes to perform
    match cli.action {
        Action::Secp256k1(secp256k1) => {
            secp256k1.generate_private_key().unwrap();
        }
    };
}

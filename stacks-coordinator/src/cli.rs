use clap::Parser;

///Command line interface for stacks coordinator
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,

    /// Config file path
    /// TODO: pull this info from sBTC
    #[arg(short, long)]
    pub config: String,

    /// Signer Config file path
    /// TODO: this should not be a seperate option really
    #[arg(short, long)]
    pub signer_config: String,

    /// Subcommand to perform
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    // Listen for incoming peg in and peg out requests.
    Run,
    // Run distributed key generation round
    Dkg,
}

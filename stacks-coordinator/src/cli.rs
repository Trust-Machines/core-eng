#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Run,
    Dkg,
}

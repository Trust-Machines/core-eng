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

#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;
    use testdir::testdir;

    #[test]
    fn secp256k1_to_stdout() {
        let mut cmd = Command::cargo_bin("stacks-signer").unwrap();

        cmd.arg("secp256k1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Generating a new private key."));
        assert_eq!(cmd.output().unwrap().stdout.len(), 190);
    }

    #[test]
    fn secp256k1_to_file() {
        let mut output_path = testdir!();
        output_path.push(".priv_key");
        assert!(!output_path.exists());

        let mut cmd = Command::cargo_bin("stacks-signer").unwrap();
        cmd.arg("secp256k1").arg("-f");
        //Test with no filename specified.
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: a value is required for",
        ));

        //Test with filename specified
        cmd.arg(output_path.to_str().unwrap_or(""));
        cmd.assert().success();
        assert!(output_path.exists());
        assert_eq!(std::fs::metadata(output_path).unwrap().len(), 66);
    }
}

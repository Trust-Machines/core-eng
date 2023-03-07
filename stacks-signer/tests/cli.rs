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
}

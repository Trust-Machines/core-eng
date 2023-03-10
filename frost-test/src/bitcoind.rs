use libc::pid_t;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use ctrlc;
use ctrlc::Signal;
use nix::sys::signal;
use nix::unistd::Pid;
use ureq;
use ureq::serde_json;
use ureq::serde_json::Value;

const BITCOIND_URL: &str = "http://abcd:abcd@localhost:18443";

pub fn bitcoind_rpc(method: &str, params: impl ureq::serde::Serialize) -> serde_json::Value {
    let rpc = ureq::json!({"jsonrpc": "1.0", "id": "tst", "method": method, "params": params});
    match ureq::post(BITCOIND_URL).send_json(&rpc) {
        Ok(response) => {
            let json = response.into_json::<serde_json::Value>().unwrap();
            let result = json.as_object().unwrap().get("result").unwrap().clone();
            println!("{} -> {}", rpc, result);
            result
        }
        Err(err) => {
            let json = err
                .into_response()
                .unwrap()
                .into_json::<serde_json::Value>()
                .unwrap();
            let err = json.as_object().unwrap().get("error").unwrap();
            println!("{} -> {}", rpc, err);
            json
        }
    }
}

pub fn bitcoind_setup() -> pid_t {
    let bitcoind_child = Command::new("bitcoind")
        .arg("-regtest")
        .arg("-rpcuser=abcd")
        .arg("-rpcpassword=abcd")
        .stdout(Stdio::null())
        .spawn()
        .expect("bitcoind failed to start");
    let bitcoind_pid = bitcoind_child.id() as pid_t;
    ctrlc::set_handler(move || {
        println!("kill bitcoind pid {:?}", bitcoind_pid);
        stop_pid(bitcoind_pid)
    })
    .expect("Error setting Ctrl-C handler");
    println!(
        "bitconind started. waiting 1 second to warm up. {}",
        BITCOIND_URL
    );
    thread::sleep(Duration::from_millis(500));
    bitcoind_pid
}

pub fn bitcoind_mine(public_key_bytes: &[u8; 33]) -> Value {
    let public_key = bitcoin::PublicKey::from_slice(public_key_bytes).unwrap();
    let address = bitcoin::Address::p2wpkh(&public_key, bitcoin::Network::Regtest).unwrap();
    bitcoind_rpc("generatetoaddress", (128, address.to_string()))
}

pub fn stop_pid(pid: pid_t) {
    signal::kill(Pid::from_raw(pid), Signal::SIGTERM).unwrap();
}

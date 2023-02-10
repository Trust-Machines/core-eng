use std::{process::Command, str::from_utf8};

use serde_json::Value;

fn main() {
    let v = Command::new("deno")
        .arg("run")
        .arg("./deno-proxy/test.mjs")
        .output()
        .unwrap()
        .stdout;
    let s = from_utf8(&v).unwrap();
    let o = serde_json::from_str::<Value>(s).unwrap();
    println!("{o:?}");
}

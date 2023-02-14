mod read_ex;
mod to_io_result;

use std::{
    io::{Error, Write},
    process::{ChildStdin, ChildStdout, Command, Stdio},
    str::from_utf8,
};

use read_ex::ReadEx;
use serde_json::{from_str, Value};
use to_io_result::ToIoResult;

pub struct Js {
    stdin: ChildStdin,
    stdout: ChildStdout,
}

impl Js {
    pub fn call(&mut self, v: Value) -> Result<Value, Error> {
        let stdin = &mut self.stdin;
        let r = v.to_string();
        stdin.write(format!("{}|{}", r.len(), r).as_bytes())?;
        stdin.flush()?;

        let stdout = &mut self.stdout;
        let len: usize = stdout.read_string_until('|')?.parse().to_io_result()?;
        let buf = stdout.read_exact_vec(len)?;
        let s = from_utf8(&buf).to_io_result()?;
        let result = from_str(s)?;
        Ok(result)
    }
}

pub fn new(path: &str) -> Result<Js, Error> {
    let mut child = Command::new("deno")
        .arg("run")
        .arg("--allow-env")
        .arg("--allow-read")
        .arg(path.to_owned() + "/test.mjs")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    Ok(Js {
        stdin: child.stdin.take().to_io_result()?,
        stdout: child.stdout.take().to_io_result()?,
    })
}

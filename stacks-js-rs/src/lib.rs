mod read_ex;
mod to_io_result;

use std::{
    io::{Error, Write},
    process::{ChildStdin, ChildStdout, Command, Stdio},
};

use read_ex::ReadEx;
use serde_json::{from_str, Value};
use to_io_result::ToIoResult;

pub struct Js {
    stdin: ChildStdin,
    stdout: ChildStdout,
}

impl Js {
    pub fn new(path: &str) -> Result<Js, Error> {
        let mut child = Command::new("deno")
            .arg("run")
            .arg("--allow-env")
            .arg("--allow-read")
            .arg(path.to_owned() + "/console.mjs")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        Ok(Js {
            stdin: child.stdin.take().to_io_result()?,
            stdout: child.stdout.take().to_io_result()?,
        })
    }
    pub fn call(&mut self, v: Value) -> Result<Value, Error> {
        let stdin = &mut self.stdin;
        let r = v.to_string();
        stdin.write(r.as_bytes())?;
        stdin.write("\n".as_bytes())?;
        stdin.flush()?;

        let stdout = &mut self.stdout;
        Ok(from_str(&stdout.read_string_until('\n')?)?)
    }
}

use std::{
    io::{Error, Write},
    process::{ChildStdin, ChildStdout, Command, Stdio},
};

use serde::{Serialize, de::DeserializeOwned};
use serde_json::{from_str, to_string};

use crate::{read_ex::ReadEx, to_io_result::TakeToIoResult};

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
            stdin: child.stdin.take_to_io_result()?,
            stdout: child.stdout.take_to_io_result()?,
        })
    }
    pub fn call<I: Serialize, O: DeserializeOwned>(&mut self, input: &I) -> Result<O, Error> {
        {
            let stdin = &mut self.stdin;
            stdin.write(to_string(input)?.as_bytes())?;
            stdin.write("\n".as_bytes())?;
            stdin.flush()?;
        }
        Ok(from_str(&self.stdout.read_string_until('\n')?)?)
    }
}

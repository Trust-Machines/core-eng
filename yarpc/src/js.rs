use std::{
    io::{self, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};

use crate::{
    read_ex::ReadEx,
    rpc::Rpc,
    to_io_result::{TakeToIoResult, ToIoResult},
};

pub struct Js {
    child: Child,
    stdin: ChildStdin,
    stdout: ChildStdout,
}

impl Drop for Js {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

impl Js {
    /// Note: the function spawns a `deno` process.
    pub fn new(path: &str) -> io::Result<Js> {
        let mut child = Command::new("deno")
            .arg("run")
            .arg("--allow-env")
            .arg("--allow-read")
            .arg("--allow-net")
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let stdin = child.stdin.take_to_io_result()?;
        let stdout = child.stdout.take_to_io_result()?;
        Ok(Js {
            child,
            stdin,
            stdout,
        })
    }
}

type JsResult<T> = Result<T, String>;

impl Rpc for Js {
    fn call<I: Serialize, O: Serialize + DeserializeOwned>(&mut self, input: &I) -> io::Result<O> {
        {
            let stdin = &mut self.stdin;
            let i = to_string(input)?;
            stdin.write_all(i.as_bytes())?;
            stdin.write_all("\n".as_bytes())?;
            stdin.flush()?;
        }
        {
            let o = self.stdout.read_string_until('\n')?;
            let result: JsResult<O> = from_str(&o)?;
            result.to_io_result()
        }
    }
}

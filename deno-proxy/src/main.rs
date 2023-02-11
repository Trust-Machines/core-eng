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

struct Js {
    stdin: ChildStdin,
    stdout: ChildStdout,
}

impl Js {
    fn call(&mut self, v: Value) -> Result<Value, Error> {
        let stdin = &mut self.stdin;
        let r = v.to_string();
        stdin.write(format!("{}|{}", r.len(), r).as_bytes())?;
        stdin.flush()?;

        let stdout = &mut self.stdout;
        let len: usize = stdout.read_string_until('|')?.parse().to_io_result()?;
        let buf = stdout.read_exact_vec(len)?;
        let s = from_utf8(&buf).to_io_result()?;
        let result = from_str::<Value>(s)?;
        Ok(result)
    }
}

fn f() -> Result<(), Error> {
    let mut child = Command::new("deno")
        .arg("run")
        .arg("./deno-proxy/test.mjs")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let stdin = child.stdin.take().to_io_result()?;
    let stdout = child.stdout.take().to_io_result()?;
    let mut js = Js { stdin, stdout };
    {
        let result = js.call(from_str("{\"b\":[],\"a\":2}")?)?;
        println!("{result}");
    }
    {
        let result = js.call(from_str("[54,null]")?)?;
        println!("{result}");
    }
    {
        let result = js.call(from_str("42")?)?;
        println!("{result}");
    }
    {
        let result = js.call(from_str("\"Hello!\"")?)?;
        println!("{result}");
    }
    {
        let result = js.call(from_str("true")?)?;
        println!("{result}");
    }
    {
        let result = js.call(from_str("null")?)?;
        println!("{result}");
    }
    Ok(())
}

fn main() {
    f().unwrap();
}

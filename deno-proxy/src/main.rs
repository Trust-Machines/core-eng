use std::{
    io::{Error, ErrorKind, Read, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    str::from_utf8,
};

use serde_json::{from_str, Value};

trait ToResult {
    type V;
    fn to_result(self) -> Result<Self::V, Error>;
}

impl<T> ToResult for Option<T> {
    type V = T;
    fn to_result(self) -> Result<Self::V, Error> {
        self.map_or(Err(Error::new(ErrorKind::InvalidData, "option")), Ok)
    }
}

impl<T, E> ToResult for Result<T, E> {
    type V = T;
    fn to_result(self) -> Result<Self::V, Error> {
        self.map_or(Err(Error::new(ErrorKind::InvalidData, "result")), Ok)
    }
}

struct Js(Child);

impl Js {
    fn call(&mut self, v: Value) -> Result<Value, Error> {
        let stdin = self.0.stdin.as_mut().to_result()?;
        let r = v.to_string();
        stdin.write(format!("{}|{}", r.len(), r).as_bytes())?;
        stdin.flush()?;

        let stdout = self.0.stdout.as_mut().to_result()?;
        let mut read_one = || -> Result<u8, Error> {
            let mut a = [0];
            stdout.read_exact(&mut a)?;
            Ok(a[0])
        };
        let mut lenStr = String::default();
        loop {
            let c = read_one()? as char;
            if c == '|' {
                break;
            }
            lenStr.push(c)
        }
        let len: usize = lenStr.parse().to_result()?;

        let mut buf = Vec::default();
        buf.resize(len, 0);
        stdout.read_exact(&mut buf)?;

        let s = from_utf8(&buf).to_result()?;
        let result = serde_json::from_str::<Value>(s)?;
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

    let mut js = Js(child);
    {
        let result = js.call(from_str("{\"a\":2}")?)?;
        println!("{result}");
    }
    {
        let result = js.call(from_str("[54]")?)?;
        println!("{result}");
    }
    Ok(())
}

fn main() {
    f().unwrap();
}

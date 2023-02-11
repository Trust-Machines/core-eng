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

struct Js {
    child: Child,
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
    let stdin = child.stdin.take().to_result()?;
    let stdout = child.stdout.take().to_result()?;
    let mut js = Js {
        child,
        stdin,
        stdout,
    };
    {
        let result = js.call(from_str("{\"a\":2}")?)?;
        println!("{result}");
    }
    {
        let result = js.call(from_str("[54]")?)?;
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

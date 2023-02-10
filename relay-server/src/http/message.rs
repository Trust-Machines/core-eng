use std::{
    collections::HashMap,
    io::{Error, Read, Write},
};

use super::{to_io_result::io_error, ToIoResult};

pub const PROTOCOL: &str = "HTTP/1.0";

pub trait Message: Sized {
    fn new(
        first_line: Vec<String>,
        headers: HashMap<String, String>,
        content: Vec<u8>,
    ) -> Result<Self, Error>;
    fn first_line(&self) -> Vec<String>;
    fn headers(&self) -> &HashMap<String, String>;
    fn content(&self) -> &Vec<u8>;

    fn read(i: &mut impl Read) -> Result<Self, Error> {
        let mut read_byte = || -> Result<u8, Error> {
            let mut buf = [0; 1];
            i.read_exact(&mut buf)?;
            Ok(buf[0])
        };

        let mut read_line = || -> Result<String, Error> {
            let mut result = String::default();
            loop {
                let b = read_byte()?;
                if b == 13 {
                    break;
                };
                result.push(b as char);
            }
            if read_byte()? != 10 {
                return Err(io_error("invalid HTTP line"));
            }
            Ok(result)
        };

        // read and parse the request line
        let first_line = read_line()?.split(' ').map(str::to_string).collect();

        // read and parse headers
        let mut content_length = 0;
        let mut headers = HashMap::default();
        loop {
            let line = read_line()?;
            if line.is_empty() {
                break;
            }
            let (name, value) = {
                let (name, value) = line.split_once(':').to_io_result("")?;
                (name.to_lowercase(), value.trim())
            };
            if name == "content-length" {
                content_length = value.parse().to_io_result("invalid content-length")?;
            } else {
                headers.insert(name, value.to_string());
            }
        }

        let mut content = vec![0; content_length];
        i.read_exact(content.as_mut_slice())?;

        // return the message
        Self::new(first_line, headers, content)
    }
    fn write(&self, o: &mut impl Write) -> Result<(), Error> {
        const EOL: &[u8] = "\r\n".as_bytes();
        const CONTENT_LENGTH: &[u8] = "content-length".as_bytes();
        const COLON: &[u8] = ":".as_bytes();

        o.write(self.first_line().join(" ").as_bytes())?;
        o.write(EOL)?;
        let mut write_header = |k: &[u8], v: &[u8]| -> Result<(), Error> {
            o.write(k)?;
            o.write(COLON)?;
            o.write(v)?;
            o.write(EOL)?;
            Ok(())
        };
        for (k, v) in self.headers().iter() {
            write_header(k.as_bytes(), v.as_bytes())?;
        }
        let content = self.content();
        let len = content.len();
        if len > 0 {
            write_header(CONTENT_LENGTH, len.to_string().as_bytes())?;
        }
        o.write(EOL)?;
        o.write(&content)?;
        Ok(())
    }
}

use std::{
    collections::HashMap,
    io::{Error, Read, Write},
};

use crate::{
    read_ex::ReadEx,
    to_io_result::{err, ToIoResult},
};

pub const PROTOCOL: &str = "HTTP/1.1";

const CONTENT_LENGTH: &str = "content-length";

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
        let mut read_line = || -> Result<String, Error> {
            let result = i.read_string_until('\r')?;
            if i.read_byte()? != 10 {
                return err("invalid HTTP line");
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
                let (name, value) = line.split_once(':').to_io_result("invalid header format")?;
                (name.to_lowercase(), value.trim())
            };
            if name == CONTENT_LENGTH {
                content_length = value.parse().to_io_result("invalid content-length")?;
            } else {
                headers.insert(name, value.to_string());
            }
        }

        let content = i.read_exact_vec(content_length)?;

        // return the message
        Self::new(first_line, headers, content)
    }
    fn write(&self, o: &mut impl Write) -> Result<(), Error> {
        const EOL: &[u8] = "\r\n".as_bytes();
        const CONTENT_LENGTH_BYTES: &[u8] = CONTENT_LENGTH.as_bytes();
        const COLON: &[u8] = ":".as_bytes();

        o.write_all(self.first_line().join(" ").as_bytes())?;
        o.write_all(EOL)?;
        let mut write_header = |k: &[u8], v: &[u8]| -> Result<(), Error> {
            o.write_all(k)?;
            o.write_all(COLON)?;
            o.write_all(v)?;
            o.write_all(EOL)?;
            Ok(())
        };
        for (k, v) in self.headers().iter() {
            write_header(k.as_bytes(), v.as_bytes())?;
        }
        let content = self.content();
        let len = content.len();
        if len > 0 {
            write_header(CONTENT_LENGTH_BYTES, len.to_string().as_bytes())?;
        }
        //These could cause partial writes. Should we check the returned number of written bytes?
        o.write_all(EOL)?;
        o.write_all(content)?;
        Ok(())
    }
}

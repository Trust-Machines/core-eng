use std::io::{Error, Write};

use super::common::Common;

pub trait Message {
    fn first_line(&self) -> String;
    fn common(&self) -> &Common;
    fn write(&self, o: &mut impl Write) -> Result<(), Error> {
        const EOL: &[u8] = "\r\n".as_bytes();
        const CONTENT_LENGTH: &[u8] = "content-length".as_bytes();
        const COLON: &[u8] = ":".as_bytes();

        o.write(self.first_line().as_bytes())?;
        o.write(EOL)?;
        let common = self.common();
        let mut write_header = |k: &[u8], v: &[u8]| -> Result<(), Error> {
            o.write(k)?;
            o.write(COLON)?;
            o.write(v)?;
            o.write(EOL)?;
            Ok(())
        };
        for (k, v) in common.headers.iter() {
            write_header(k.as_bytes(), v.as_bytes())?;
        }
        let content = &common.content;
        let len = content.len();
        if len > 0 {
            write_header(CONTENT_LENGTH, len.to_string().as_bytes())?;
        }
        o.write(EOL)?;
        o.write(&content)?;
        Ok(())
    }
}

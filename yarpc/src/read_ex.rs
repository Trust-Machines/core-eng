use std::io::{Error, Read};

pub trait ReadEx: Read {
    fn read_byte(&mut self) -> Result<u8, Error> {
        let mut a = [0];
        self.read_exact(&mut a)?;
        Ok(a[0])
    }
    fn read_string_until(&mut self, b: char) -> Result<String, Error> {
        let mut buf = String::default();
        loop {
            let c = self.read_byte()? as char;
            if c == b {
                break;
            }
            buf.push(c)
        }
        Ok(buf)
    }
    fn read_exact_vec(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0; len];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl<T: Read> ReadEx for T {}

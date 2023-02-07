use std::io::{Write, Error};

use super::common::Common;

trait Message {
    fn common(&self) -> &Common;
    fn write(&self, o: &mut impl Write) -> Result<(), Error> {
        const EOL: &str = "\r\n"; 
        let common = self.common();
        let mut write = |text: &str| o.write(text.as_bytes());
        for (k, v) in common.headers.iter() {
            write(k.as_str())?;
            write(":")?;
            write(v.as_str())?;
            write(EOL)?;
        }
        write(EOL)?;
        o.write(&common.content)?;
        Ok(())
    }
}

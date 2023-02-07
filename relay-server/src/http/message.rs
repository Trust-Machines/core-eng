use std::io::{Write, Error};

trait Message {
    fn lines(&self) -> Vec<String>;
    fn content(&self) -> &[u8];
    fn write(&self, o: &mut impl Write) -> Result<(), Error> {
        const EOL: &str = "\r\n"; 
        let mut write = |text: &str| o.write(text.as_bytes());
        for line in self.lines() {
            write(line.as_str())?;
            write(EOL)?;
        }
        write(EOL)?;
        o.write(&self.content())?;
        Ok(())
    }
}

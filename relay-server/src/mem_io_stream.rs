use std::io::Cursor;

use crate::io_stream::IoStream;

pub struct MemIoStream<'a> {
    pub i: Cursor<&'a [u8]>,
    pub o: Cursor<&'a mut Vec<u8>>,
}

pub trait MemIoStreamEx<'a> {
    fn mem_io_stream(self, output: &'a mut Vec<u8>) -> MemIoStream<'a>;
}

impl<'a> MemIoStreamEx<'a> for &'a [u8] {
    fn mem_io_stream(self, output: &'a mut Vec<u8>) -> MemIoStream<'a> {
        MemIoStream {
            i: Cursor::new(self),
            o: Cursor::new(output),
        }
    }
}

impl<'a> IoStream for MemIoStream<'a> {
    type Read = Cursor<&'a [u8]>;
    type Write = Cursor<&'a mut Vec<u8>>;
    fn istream(&mut self) -> &mut Self::Read {
        &mut self.i
    }
    fn ostream(&mut self) -> &mut Self::Write {
        &mut self.o
    }
}

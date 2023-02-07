use std::io::Cursor;

use crate::IoStream;

pub struct MemIoStream<'a> {
    pub i: Cursor<&'a [u8]>,
    pub o: Cursor<Vec<u8>>,
}

pub trait MemIoStreamEx<'a> {
    fn mem_io_stream(self) -> MemIoStream<'a>;
}

impl<'a> MemIoStreamEx<'a> for &'a [u8] {
    fn mem_io_stream(self) -> MemIoStream<'a> {
        MemIoStream {
            i: Cursor::new(self),
            o: Default::default(),
        }
    }
}

impl<'a> MemIoStreamEx<'a> for &'a str {
    fn mem_io_stream(self) -> MemIoStream<'a> {
        self.as_bytes().mem_io_stream()
    }
}

impl<'a> IoStream for MemIoStream<'a> {
    type Read = Cursor<&'a [u8]>;
    type Write = Cursor<Vec<u8>>;
    fn istream(&mut self) -> &mut Self::Read {
        &mut self.i
    }
    fn ostream(&mut self) -> &mut Self::Write {
        &mut self.o
    }
}

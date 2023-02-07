use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub trait IoStream {
    type Read: Read;
    type Write: Write;
    fn istream(&mut self) -> &mut Self::Read;
    fn ostream(&mut self) -> &mut Self::Write;
}

impl IoStream for TcpStream {
    type Read = TcpStream;
    type Write = TcpStream;
    fn istream(&mut self) -> &mut Self::Read {
        self
    }
    fn ostream(&mut self) -> &mut Self::Write {
        self
    }
}

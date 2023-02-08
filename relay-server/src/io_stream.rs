use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::http::{Message, Request, Response};

/// A trait for bidirectional stream.
///
/// For example, `TcpStream` is a bidirectional stream.
pub trait IoStream: Sized {
    type Read: Read;
    type Write: Write;
    fn istream(&mut self) -> &mut Self::Read;
    fn ostream(&mut self) -> &mut Self::Write;
    fn call(mut self, request: Request) -> Response {
        let o = self.ostream();
        // send data to a callee.
        request.write(o).unwrap();
        // make sure we deliver all data to to the callee.
        o.flush().unwrap();
        // read data from the callee.
        Response::read(self.istream()).unwrap()
    }
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

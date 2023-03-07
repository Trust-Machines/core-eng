use std::{
    io::{Error, Read, Write},
    net::TcpStream,
};

use crate::http::{Call, Message, Request, Response};

/// A trait for bidirectional stream.
///
/// For example, `TcpStream` is a bidirectional stream.
pub trait IoStream: Sized {
    type Read: Read;
    type Write: Write;
    fn istream(&mut self) -> &mut Self::Read;
    fn ostream(&mut self) -> &mut Self::Write;
}

impl<T: IoStream> Call for T {
    fn call(&mut self, request: Request) -> Result<Response, Error> {
        let o = self.ostream();
        // send data to a callee.
        request.write(o)?;
        // make sure we deliver all data to to the callee.
        o.flush()?;
        // read data from the callee.
        Response::read(self.istream())
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

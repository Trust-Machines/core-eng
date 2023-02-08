use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::http::{Request, Response, Message};

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
        request.write(o).unwrap();        
        o.flush().unwrap();
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

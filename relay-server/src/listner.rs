use std::{net::{Incoming, TcpListener, TcpStream}, io::Error};

use crate::{state::State, server::{Stream, ServerEx}};

pub trait Listner {
    type Stream: Stream;
    type Incoming<'a>: IntoIterator<Item = Result<Self::Stream, Error>>
    where
        Self: 'a;
    fn into_incoming<'a>(&'a self) -> Self::Incoming<'a>;
    fn run(&self) {
        let mut state = State::default();
        for stream_or_error in self.into_incoming() {
            let f = || stream_or_error?.update_state(&mut state);
            if let Err(e) = f() {
                eprintln!("IO error: {e}");
            }
        }
    }    
}

impl Listner for TcpListener {
    type Stream = TcpStream;
    type Incoming<'a> = Incoming<'a>;
    fn into_incoming<'a>(&'a self) -> Self::Incoming<'a> {
        self.incoming()
    }
}

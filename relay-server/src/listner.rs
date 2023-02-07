use std::io::Error;

use crate::{io_stream::IoStream, server::Server};

pub fn run_server<T: IoStream>(i: &mut impl Iterator<Item = Result<T, Error>>) {
    let mut state = Server::default();
    for stream_or_error in i {
        let f = || state.update_state(&mut stream_or_error?);
        if let Err(e) = f() {
            eprintln!("IO error: {e}");
        }
    }
}

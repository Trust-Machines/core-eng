use std::{io::Error, net::TcpListener};

use relay_server::Server;
use yarpc::http::IoStream;

pub fn run_server<T: IoStream>(i: &mut impl Iterator<Item = Result<T, Error>>) {
    let mut server = Server::default();
    for stream_or_error in i {
        let f = || server.update(&mut stream_or_error?);
        if let Err(e) = f() {
            eprintln!("IO error: {e}");
        }
    }
}

fn main() {
    let addr = "127.0.0.1:9776";
    let listner = TcpListener::bind(addr).unwrap();
    println!("Listening {addr}...");
    run_server(&mut listner.incoming());
}

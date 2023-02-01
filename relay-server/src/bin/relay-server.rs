use std::net::TcpListener;

use relay_server::run_server;

fn main() {
    let addr = "127.0.0.1:9776";
    let listner = TcpListener::bind(addr).unwrap();
    println!("Listening {addr}...");
    run_server(&mut listner.incoming());
}

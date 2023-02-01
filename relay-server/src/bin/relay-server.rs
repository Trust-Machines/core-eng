use std::net::TcpListener;

use relay_server::run_server;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:9776").unwrap();
    run_server(&mut listner.incoming());
}
use std::net::TcpListener;

use state::State;

mod state;
mod server;
mod http;
mod to_io_result;
mod url;
mod listner;

use crate::listner::Listner;

fn main() {
    TcpListener::bind("127.0.0.1:9776").unwrap().run();
}
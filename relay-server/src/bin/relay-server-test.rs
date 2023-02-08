use std::{net::TcpStream};

use relay_server::{IoStream, Request, Response, RemoteState, State};

fn call(request: Request) -> Response {
    TcpStream::connect("127.0.0.1:9776").unwrap().call(request)
}

fn main() {
    let mut state = RemoteState(call);
    assert!(state.get("a".to_string()).is_empty());
    let msg0 = "Msg1".as_bytes();
    state.post(msg0.to_vec());
    assert_eq!(state.get("a".to_string()), msg0);
}

use std::{net::TcpStream, thread::yield_now};

use relay_server::{IoStream, RemoteState, Request, Response, State};

const ADDR: &str = "127.0.0.1:9776";

fn call(request: Request) -> Response {
    TcpStream::connect(ADDR).unwrap().call(request)
}

fn main() {
    // waiting for a server
    while TcpStream::connect(ADDR).is_err() {
        yield_now()
    }
    //
    let mut state = RemoteState(call);
    //
    assert!(state.get(1.to_string()).is_empty());
    assert!(state.get(3.to_string()).is_empty());
    // assert_eq!(0, state.highwaters.len());
    state.post("Msg # 0".as_bytes().to_vec());
    assert_eq!("Msg # 0".as_bytes().to_vec(), state.get(1.to_string()));
    assert_eq!("Msg # 0".as_bytes().to_vec(), state.get(5.to_string()));
    assert_eq!("Msg # 0".as_bytes().to_vec(), state.get(4.to_string()));
    assert!(state.get(1.to_string()).is_empty());
    state.post("Msg # 1".as_bytes().to_vec());
    assert_eq!("Msg # 1".as_bytes().to_vec(), state.get(1.to_string()));
    assert_eq!("Msg # 0".as_bytes().to_vec(), state.get(3.to_string()));
    assert_eq!("Msg # 1".as_bytes().to_vec(), state.get(5.to_string()));
    state.post("Msg # 2".as_bytes().to_vec());
    assert_eq!("Msg # 2".as_bytes().to_vec(), state.get(1.to_string()));
    assert_eq!("Msg # 1".as_bytes().to_vec(), state.get(4.to_string()));
    assert_eq!("Msg # 2".as_bytes().to_vec(), state.get(4.to_string()));
    println!("passed");
}

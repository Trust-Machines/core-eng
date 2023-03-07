use std::{io::Error, net::TcpStream, thread::yield_now};

use relay_server::{ProxyState, State};
use yarpc::http::{Call, Request, Response};

const ADDR: &str = "127.0.0.1:9776";

struct RemoteServer();

impl Call for RemoteServer {
    fn call(&mut self, request: Request) -> Result<Response, Error> {
        // note: `relay-server` doesn't support multiple requests in one connection
        // so we create a new connection every time when we send a request.
        TcpStream::connect(ADDR).unwrap().call(request)
    }
}

fn main() {
    // waiting for a server
    while TcpStream::connect(ADDR).is_err() {
        yield_now()
    }
    //
    let mut state = ProxyState(RemoteServer());
    //
    assert!(state.get(1.to_string()).unwrap().is_empty());
    assert!(state.get(3.to_string()).unwrap().is_empty());
    state.post("Msg # 0".as_bytes().to_vec()).unwrap();
    assert_eq!(
        "Msg # 0".as_bytes().to_vec(),
        state.get(1.to_string()).unwrap()
    );
    assert_eq!(
        "Msg # 0".as_bytes().to_vec(),
        state.get(5.to_string()).unwrap()
    );
    assert_eq!(
        "Msg # 0".as_bytes().to_vec(),
        state.get(4.to_string()).unwrap()
    );
    assert!(state.get(1.to_string()).unwrap().is_empty());
    state.post("Msg # 1".as_bytes().to_vec()).unwrap();
    assert_eq!(
        "Msg # 1".as_bytes().to_vec(),
        state.get(1.to_string()).unwrap()
    );
    assert_eq!(
        "Msg # 0".as_bytes().to_vec(),
        state.get(3.to_string()).unwrap()
    );
    assert_eq!(
        "Msg # 1".as_bytes().to_vec(),
        state.get(5.to_string()).unwrap()
    );
    state.post("Msg # 2".as_bytes().to_vec()).unwrap();
    assert_eq!(
        "Msg # 2".as_bytes().to_vec(),
        state.get(1.to_string()).unwrap()
    );
    assert_eq!(
        "Msg # 1".as_bytes().to_vec(),
        state.get(4.to_string()).unwrap()
    );
    assert_eq!(
        "Msg # 2".as_bytes().to_vec(),
        state.get(4.to_string()).unwrap()
    );
    println!("passed");
}

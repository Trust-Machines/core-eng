# A Relay Server

The `relay-server` is an HTTP service that allows clients to send messages and retrieve the messages in the order they were received. The messages are stored on the server, and the clients are identified by an `id` that is passed in the URL of the request.

It has two functions:

- Accepting messages and storing all of them. `POST` method. 
  For example, `curl 'http://127.0.0.1:9776' -X POST -d 'message'`. 
- Returning the messages in the same order as received for each client. 
  For example, `curl 'http://127.0.0.1:9776/?id=alice'`. 

## Installation (optional)

The server can be installed using the command

```sh
cargo install relay-server --git https://github.com/Trust-Machines/core-eng
```

## Start the `relay-server` server

To start the server, you can run `cargo run relay-server` from the root of the repository, 
or simply `relay-server` if it's installed. The default address for the server is http://127.0.0.1:9776.

## Integration Test

1. Start the server `cargo run relay-server`
2. Run [./test.sh](./test.sh) in another terminal.
3. Close the server using `Ctrl+C`.

## Using as a library

In addition to being used as a standalone server, the `relay-server` can also be used as a library in your own Rust projects. The library is designed not to use IO directly, and all IO bindings are moved to executables.

### As a local server

The `Server` type can be used as in memory server. By default, it doesn't listen any port. A user 
should bind it with IO.

```rust
use relay_server::{Call, Method, Server, Response};

let mut server = Server::default();
// send a message "Hello!"
{
    let request = Method::POST.request(
        "/".to_string(),
        Default::default(),
        "Hello!".as_bytes().to_vec(),
    );
    let response = server.call(request).unwrap();
    let expected = Response::new(
        200,
        "OK".to_string(),
        Default::default(),
        Default::default(),
    );
    assert_eq!(response, expected);
}
```

See also [src/bin/relay-server.rs](src/bin/relay-server.rs) as an example.

### Using `State` trait

There are two implementations of the `State` trait:
- `MemState` keeps a relay-server state in memory.
- `ProxyState` is a proxy object which communicates with provided IO to 
  get and update a remote `relay-server`.

```rust
use relay_server::{MemState, State};

let mut mem_state = MemState::default();

mem_state.post(b"Hello world!".to_vec());

let message = mem_state.get("node".to_string()).unwrap();

assert_eq!(message, b"Hello world!".to_vec());
```
# Deno Proxy

## Deno Installation

```sh
cargo install deno
```

## Run JS Test

```sh
deno run --allow-env --allow-read ./deno-proxy/test.mjs
```

## Run Rust Test

```sh
cargo run --bin deno-proxy
```

## Protocol

The program communicates with `test.mjs` using STDIO. All messages should be ASCII symbols.

Each message contains 

- a lenght of a JSON part of the message,
- a '|' symbol,
- a JSON part of the message.

### Examples

- one message 
  ```
  8|{"a":42}
  ```
- multiple messages
  ```
  8|{"a":42}11|[0,-1,true]
  ```

# Deno Proxy

## Deno Installation

```sh
cargo install deno
```

## Protocol

The program communicates with `test.mjs` using STDIO. All messages should be ASCII symbols.

Each message contains 

- a JSON part of the message,
- `\n` symbol.

### Examples

- one message 
  ```
  {"a":42}\n
  ```
- multiple messages
  ```
  {"a":42}\n[0,-1,true]\n
  ```

## Debugging the `deno-proxy`

### Run Rust `deno-proxy`

```sh
cargo run --bin deno-proxy
```

### Run JS `deno-proxy`

```sh
deno run --allow-env --allow-read ./deno-proxy/test.mjs
```

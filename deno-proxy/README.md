# Deno Proxy

It's a simple example how to make RPC (remote procedure call) from Rust to JS.

## Deno Installation

```sh
cargo install deno
```

## Protocol

The program communicates with `test.mjs` using STDIO.

Each message contains 

- a JSON part of the message. **Note:** the JSON shouldn't contain `\n` symbols.
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

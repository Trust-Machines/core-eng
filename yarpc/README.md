# YARPC (Yet Another RPC) Library

A simple RPC (remote procedure call) from Rust to JS using STDIO.

## Deno Installation

```sh
cargo install deno
```

## Protocol

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

### Running `mirror.ts` test

```sh
deno run --allow-env --allow-read ./yarpc/js/test/mirror.ts
```

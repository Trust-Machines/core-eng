# Stacks-js-rs

Partial Implementation of Stacks on Rust using `stacks.js`. We use a simple 
RPC (remote procedure call) from Rust to JS using STDIO.

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

## Debugging the `stacks-js-rs`

### Running `stacks-js-rs`

```sh
cargo run --bin stacks-js-rs
```

### Running `console.mjs`

```sh
deno run --allow-env --allow-read ./stacks-js-rs/console.mjs
```

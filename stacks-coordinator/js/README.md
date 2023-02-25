# Make Contract Call

Calling `makeContractCall` function from `stacks.js`. We use a simple
RPC (remote procedure call) from Rust to JS using STDIO.

## Example

Run
```sh
deno run --allow-env --allow-read --allow-net ./stacks-js-rs/js/make_contract_call.ts
```

and enter

```json
{"senderKey":"0001020304050607080910111213141516171819202122232425262728293031","contractAddress":"SPBMRFRPPGCDE3F384WCJPK8PQJGZ8K9QKK7F59X","contractName":"","functionName":"mint","functionArgs":[],"anchorMode":3,"fee":0}
```

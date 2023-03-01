# Stacks/Transactions Contract Call

Calling any function from `@stacks/transactions`.

## Example

Run
```sh
deno run --allow-env --allow-read --allow-net ./yarpc/js/stacks/transactions.ts
```

and enter

```json
["makeContractCall",{"senderKey":"0001020304050607080910111213141516171819202122232425262728293031","contractAddress":"SPBMRFRPPGCDE3F384WCJPK8PQJGZ8K9QKK7F59X","contractName":"","functionName":"mint","functionArgs":[],"anchorMode":3,"fee":0}]
```

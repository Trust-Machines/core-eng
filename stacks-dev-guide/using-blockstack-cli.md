# Using the blockstack-cli tool

This guide shows how to use the `blockstack-cli` tool from `stacks-network` to publish a contract and call a function using the Stacks 2.1 testnet.

## Instructions to run the test

Clone the stacks repository and build the project
```
gh repo clone stacks-network/stacks-blockchain && cd stacks-blockchain
cargo build -r
PATH=$PATH:$(pwd)/target/release
```

Learn about the `blockstack-cli` tool:
```
blockstack-cli --help
blockstack-cli generate-sk --help
blockstack-cli contract-call --help
blockstack-cli publish --help
```

Define some values to be used
```
# The 2.1 API service
API_URL="https://2-1-api.testnet.hiro.so"

# The transaction fee rate used. Increase this value if your transactions are not being mined.
FEE_RATE=500
```

Generate a testnet private key
```
blockstack-cli --testnet generate-sk | tee sk.json
SECRET_KEY=$(jq -r .secretKey sk.json)
STX_ADDRESS=$(jq -r .stacksAddress sk.json)
```

Request STX tokens from the faucet
```
curl -s -X POST $API_URL/extended/v1/faucets/stx -d "address=$STX_ADDRESS" | tee faucet-response.json

{
  "success": true,
  "txId": "0x5f75c500007446a334ecd4744966703bda75109b309f5207d3963798149d600b",
  "txRaw": "80800000000400c2963cbcd9cf3f60311c34087ceb45c8abdbc413000000000000000b00000000000000b400005c0f82d588d70e9b261c3f1fd7d9e8bf0eacb98056f273bb59896121f23598ab0a3772d7b42c6958ff7495f07f240f8986b8339bef3463b71e4585791c45c98e03020000000000051a509b3cebec1ce95e7215039d8acd32d344d70e7c000000001dcd650046617563657400000000000000000000000000000000000000000000000000000000"
}
```

Wait a few minutes until the transfer is confirmed by the network.
```
watch -n1 "curl -s $API_URL/extended/v1/tx/$(jq -r .txId faucet-response.json) |  jq .tx_status"
```

Check faucet transaction status:
```
curl -s $API_URL/extended/v1/tx/$(jq -r .txId faucet-response.json) |  jq .tx_status
```

Check the account balance:
```
curl -s $API_URL/extended/v1/address/$STX_ADDRESS/stx | jq -r .balance

500000000
```

Write the test contract source code:
```
tee contract.clar << END
(define-data-var data int 1)

(define-public (the-method)
  (ok true)) ;; A trivial method
END
```

Get the next nonce for our account
```
NEXT_NONCE=$(curl -s $API_URL/extended/v1/address/$STX_ADDRESS/nonces | jq -r .possible_next_nonce)
```

Generate the transaction to publish the test contract
```
blockstack-cli --testnet publish $SECRET_KEY $FEE_RATE $NEXT_NONCE the-contract contract.clar \
  | tee contract-publish.txn
```

Broadcast the transaction to publish the test contract
```
cat contract-publish.txn | xxd -r -p \
  | curl -s $API_URL/v2/transactions -X POST --data-binary @- -H "content-type:application/octet-stream" \
  | tee contract-publish.json
```

Wait a few minutes until the contract publish transaction is confirmed:
```
curl -sL $API_URL/extended/v1/tx/$(jq -r . contract-publish.json) |  jq .tx_status
```

Generate the transaction to call the contract
```
NEXT_NONCE=$(curl -s $API_URL/extended/v1/address/$STX_ADDRESS/nonces | jq -r .possible_next_nonce)

blockstack-cli --testnet contract-call $SECRET_KEY $FEE_RATE $NEXT_NONCE $STX_ADDRESS the-contract the-method \
  | tee contract-call.txn
```

Broadcast the transaction to call the contract
```
cat contract-call.txn | xxd -r -p \
  | curl -s $API_URL/v2/transactions -X POST --data-binary @- -H "content-type:application/octet-stream" \
  | tee contract-call.json
```

Wait a few minutes until the contract call transaction is confirmed.
Verify the contract call throws a `CheckError` and that the transaction fee was charged.
```
curl -sL $API_URL/extended/v1/tx/$(jq -r . contract-call.json)
```

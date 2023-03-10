## Frost-Coordinator

## Sample run

3 signers, message: [1,2,3,4]

in separate terminals run:
```
relay-server $ cargo run
frost-signer $ cargo run -- --id 3
frost-signer $ cargo run -- --id 2
frost-signer $ cargo run -- --id 1
frost-coordinator $ cargo run dkg-sign -- 1 2 3 4

```

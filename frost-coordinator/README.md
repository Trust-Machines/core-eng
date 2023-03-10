## Frost-Coordinator

## Sample run

3 signers, message: [1,2,3,4]

in separate terminals run:
```

relay-server $ cargo run
frost-signer $ cargo run -- --id 3 --config conf/signer.toml
frost-signer $ cargo run -- --id 2 --config conf/signer.toml
frost-signer $ cargo run -- --id 1 --config conf/signer.toml
frost-coordinator $ cargo run -- --config ../frost-signer/conf/signer.toml dkg-sign -- 1 2 3 4

```

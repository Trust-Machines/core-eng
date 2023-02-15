This intergration test runs through the sBTC process 
of signer coordination, btc peg-in, and btc peg-out.

## Test setup
The integration test is a rust function that calls into the
signer and coordinator crates while assuming there is a network
but not actually simulating one. Each crate's api 
should be divisible enough to run in a single threaded test,
meaning it can avoid any crate-internal main-loops waiting on network
activity.

# Integration Test

## Part 1 DKG
* coordinator signals begin DKG
* signers generate PolyCommitments/Public key-shares.
* signers generate Private key-shares.
* signers flood-send all private and public key-shares
* signers gather the public and private key-shares they dont already have
* coordinator gathers all public key-shares
* coordinator computes peg-wallet-address

## Part 2 Peg In

* user generates BTC TX from user to peg-wallet-address
  * output 1: P2PKH of peg-wallet-address
  * output 2: OP_RETURN <stx-address>
* user publishes BTC TX

## Part 3 Peg Out
* coordinator requests and gathers nonces from the signers
* coordinator generates BTC TX
  * output 1: P2PKH of user-address
* coordinator flood-send signtaure request of BTC TX payload
* signers respond with signature-share
* coordinator gathers signature-shares from the signers
* coordinator publishes BTC TX


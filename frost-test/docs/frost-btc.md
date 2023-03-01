This intergration test runs through the sBTC process 
of signer coordination, btc peg-in, and btc peg-out.

## Test setup
The integration test is a rust function that calls into the
signer and coordinator crates while assuming there is a network
but not actually simulating one. Each crate's api 
should be divisible enough to run in a single threaded test,
meaning it can avoid any crate-internal main-loops that wait on
network activity.

# Integration Test

## Part 1 DKG
* coordinator signals begin DKG
* signers generate PolyCommitments/Public key-shares.
* signers generate Private key-shares.
* signers flood-send all private and public key-shares
* signers gather the public and private key-shares they dont already have
* coordinator gathers all public key-shares
* coordinator computes peg-wallet-address

## Part 2 Peg In with OP_RETURN

* user creates Peg-In BTC TX
  * output 1: OP_RETURN "<" <stx-address> <contract> <memo>
  * output 2: P2TR of peg-wallet-address
* user publishes BTC TX

## Part 3 Peg Out
* user creates Peg-Out request
  * output 1: OP_RETURN ">" <amount> <signature> <memo>
  * output 2: P2PKH user-bitcoin-address
* coordinator requests and gathers nonces from the signers
* coordinator generates Peg-Out Fulfilment BTC TX
  * output 1: OP_RETURN ">" <amount> <signature> <memo>
  * output 2: P2PKH user-bitcoin-address 
* coordinator flood-send signtaure request of BTC TX payload
* signers respond with signature-share
* coordinator gathers signature-shares from the signers
* coordinator publishes BTC TX


### Notes

* Why P2TR(Taproot)? The process depends on FROST which generates a Schnorr signtaure. Only a Taproot output (segwit v1) can be spent by an input with a Schnorr signture.
* BIP340 takes a shortcut with R of Signature (R,s) by using only the X part and implicitly choosing the Y coordinate that is even. This makes signatures always 64 bytes long.

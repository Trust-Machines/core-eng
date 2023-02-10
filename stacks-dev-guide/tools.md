# A list of useful tools for our work

- [btcdeb](https://github.com/bitcoin-core/btcdeb)
- [cargo-nextest](https://nexte.st/)

## Cargo Nextest
Cargo nextest is a near drop-in replacement for libtest (what you use when calling `cargo test`).
It gives you prettier output and improved parallelism for running tests.

It can be installed by
```
cargo install cargo-nextest --locked
```

and later invoked by replacing `cargo test` with `cargo nextest run`.
For a more comprehensive guide of CLI arguments, [rtfm](https://nexte.st/book/usage.html).

## Btcdeb
Btcdeb is a useful tool for debugging Bitcoin scripts. For example, to validate a transaction signature, do
```
> $ 
btcdeb --tx=02000000013a9bad9b0bb8859bd22811be544269ee34f272c1185deb0761cb9aefcbfbfdc2000000006a47304402200cc8b0471a38edad2ff9f9799521b7d948054817793c980eaf3a6637ddfb939702201c1a801461d4c3cf4de4e7336454dba0dd70b89d71f221e991cb6a79df1a860d012102ce9f5972fe1473c9b6948949f676bbf7893a03c5b4420826711ef518ceefd8dcfeffffff0226f20b00000000001976a914d138551aa10d1f891ba02689390f32ce09b71c1788ac28b0ed01000000001976a914870c7d8085e1712539d8d78363865c42d2b5f75a88ac5b880800 '[OP_DUP OP_HASH160 1290b657a78e201967c22d8022b348bd5e23ce17 OP_EQUALVERIFY OP_CHECKSIG ]' 304402200cc8b0471a38edad2ff9f9799521b7d948054817793c980eaf3a6637ddfb939702201c1a801461d4c3cf4de4e7336454dba0dd70b89d71f221e991cb6a79df1a860d01 02ce9f5972fe1473c9b6948949f676bbf7893a03c5b4420826711ef518ceefd8dc

btcdeb 0.4.22 -- type `btcdeb -h` for start up options
LOG: signing segwit taproot
notice: btcdeb has gotten quieter; use --verbose if necessary (this message is temporary)
5 op script loaded. type `help` for usage information
script                                   |                                                             stack
-----------------------------------------+-------------------------------------------------------------------
OP_DUP                                   | 02ce9f5972fe1473c9b6948949f676bbf7893a03c5b4420826711ef518ceefd8dc
OP_HASH160                               | 304402200cc8b0471a38edad2ff9f9799521b7d948054817793c980eaf3a663...
1290b657a78e201967c22d8022b348bd5e23ce17 |
OP_EQUALVERIFY                           |
OP_CHECKSIG                              |
#0000 OP_DUP
btcdeb> step
		<> PUSH stack 02ce9f5972fe1473c9b6948949f676bbf7893a03c5b4420826711ef518ceefd8dc
script                                   |                                                             stack
-----------------------------------------+-------------------------------------------------------------------
OP_HASH160                               | 02ce9f5972fe1473c9b6948949f676bbf7893a03c5b4420826711ef518ceefd8dc
1290b657a78e201967c22d8022b348bd5e23ce17 | 02ce9f5972fe1473c9b6948949f676bbf7893a03c5b4420826711ef518ceefd8dc
OP_EQUALVERIFY                           | 304402200cc8b0471a38edad2ff9f9799521b7d948054817793c980eaf3a663...
OP_CHECKSIG                              |
#0001 OP_HASH160
btcdeb> step
		<> POP  stack
		<> PUSH stack 1290b657a78e201967c22d8022b348bd5e23ce17
script                                   |                                                             stack
-----------------------------------------+-------------------------------------------------------------------
1290b657a78e201967c22d8022b348bd5e23ce17 |                           1290b657a78e201967c22d8022b348bd5e23ce17
OP_EQUALVERIFY                           | 02ce9f5972fe1473c9b6948949f676bbf7893a03c5b4420826711ef518ceefd8dc
OP_CHECKSIG                              | 304402200cc8b0471a38edad2ff9f9799521b7d948054817793c980eaf3a663...
#0002 1290b657a78e201967c22d8022b348bd5e23ce17
btcdeb> step
		<> PUSH stack 1290b657a78e201967c22d8022b348bd5e23ce17
script                                   |                                                             stack
-----------------------------------------+-------------------------------------------------------------------
OP_EQUALVERIFY                           |                           1290b657a78e201967c22d8022b348bd5e23ce17
OP_CHECKSIG                              |                           1290b657a78e201967c22d8022b348bd5e23ce17
                                         | 02ce9f5972fe1473c9b6948949f676bbf7893a03c5b4420826711ef518ceefd8dc
                                         | 304402200cc8b0471a38edad2ff9f9799521b7d948054817793c980eaf3a663...
#0003 OP_EQUALVERIFY
btcdeb> step
		<> POP  stack
		<> POP  stack
		<> PUSH stack 01
		<> POP  stack
script                                   |                                                             stack
-----------------------------------------+-------------------------------------------------------------------
OP_CHECKSIG                              | 02ce9f5972fe1473c9b6948949f676bbf7893a03c5b4420826711ef518ceefd8dc
                                         | 304402200cc8b0471a38edad2ff9f9799521b7d948054817793c980eaf3a663...
#0004 OP_CHECKSIG
btcdeb> step
EvalChecksig() sigversion=0
Eval Checksig Pre-Tapscript
GenericTransactionSignatureChecker::CheckECDSASignature(71 len sig, 33 len pubkey, sigversion=0)
  sig         = 304402200cc8b0471a38edad2ff9f9799521b7d948054817793c980eaf3a6637ddfb939702201c1a801461d4c3cf4de4e7336454dba0dd70b89d71f221e991cb6a79df1a860d01
  pub key     = 02ce9f5972fe1473c9b6948949f676bbf7893a03c5b4420826711ef518ceefd8dc
  script code = 76a9141290b657a78e201967c22d8022b348bd5e23ce1788ac
  hash type   = 01 (SIGHASH_ALL)
SignatureHash(nIn=0, nHashType=01, amount=33159830)
- sigversion = SIGVERSION_BASE (non-segwit style)
 << txTo.vin[nInput=0].prevout = COutPoint(c2fdfbcbef, 0)
(SerializeScriptCode)
 << scriptCode.size()=25 - nCodeSeparators=0
 << script:76a9141290b657a78e201967c22d8022b348bd5e23ce1788ac
 << txTo.vin[nInput].nSequence = 4294967294 [0xfffffffe]
  sighash     = 138ae80f8df5d0fea9bf0cd48729ead6a8f98454f5a611818090a47f063fa905
  pubkey.VerifyECDSASignature(sig=304402200cc8b0471a38edad2ff9f9799521b7d948054817793c980eaf3a6637ddfb939702201c1a801461d4c3cf4de4e7336454dba0dd70b89d71f221e991cb6a79df1a860d, sighash=138ae80f8df5d0fea9bf0cd48729ead6a8f98454f5a611818090a47f063fa905):
  result: success
		<> POP  stack
		<> POP  stack
		<> PUSH stack 01
script                                   |                                                             stack
-----------------------------------------+-------------------------------------------------------------------
                                         |                                                                 01
```

# Stacks Developer Guide

## What is the structure of the stacks-network repo?

[https://github.com/stacks-network/stacks-blockchain](https://github.com/stacks-network/stacks-blockchain)

Branches

- `master` - For hotfixes, branch off of master.
- `next` - For consensus breaking changes, branch off of the next branch.
- `develop` - For typical development, branch off of the develop branch.

Cargo packages

| Package Name | Cargo file location | Description
| --- | --- | ---
| `blockstack-core` | `/Cargo.toml` |
| `clarity` | `/clarity/Cargo.toml` |
| `stacks-node` | `/testnet/stacks-node/Cargo.toml` |
| `stacks-common` | `/stacks-common/Cargo.toml` |
| `stx-genesis` | `/stx-genesis/Cargo.toml` |
| `puppet-chain` | `/tools/puppet-chain/Cargo.toml` |

## How do I run the test suite?

### Local tests

```
cargo test --release --workspace -- --report-time -Z unstable-options
```

### Integration tests

- Download Bitcoin <https://bitcoin.org/bin/bitcoin-core-0.20.0/>
- The list of integration tests is here: <https://github.com/stacks-network/stacks-blockchain/blob/8da076e7c72f3d743caecfd12a9b32496f0a236c/.github/workflows/bitcoin-tests.yml#L41>
- All integration tests are tagged with `#[ignore]`

```
PATH=$PATH:/path-to/bitcoin-22.0/bin
BITCOIND_TEST=1
cargo test -p stacks-node -- --ignored "tests::neon_integrations::miner_submit_twice"
```

### Clarity core contract tests
- [Install clarinet](https://book.clarity-lang.org/ch01-01-installing-tools.html)
- Run the following command from the project root
```
clarinet test --coverage --manifest-path=./contrib/core-contract-tests/Clarinet.toml
```

## What are the different burnchain modes and run loops?

| Burnchain Mode | Run Loop | Bitcoin Network | Nickname | Notes |
| --- | --- | --- | --- | ---
| mainnet | Neon | mainnet | mainnet
| xenon | Neon | testnet | testnet
| neon | Neon | regtest | local
| krypton | Neon | regtest | regtest
| mocknet | Helium | regtest | mocknet
| helium | Helium | regtest |
| argon | Neon | regtest | | Unsupported by stacks-node

- Run Loops
  - Neon - Uses a real burnchain node (Bitcoin)
  - Heium - Simulated burnchain and some simulated nodes taking turns in producing blocks.

Do not confuse burnchain modes **neon** and **helium** with run loops **Neon** and **Helium**.

## How do I run a mock miner?

## What is testnet?

## What is sortition?
Sortition is the process of selecting a miner principal to broadcast block data
to the stacks nodes for the next winning block. Before candidating, miners must
commit the chain tip they're building on and the hash of the block they intend
to broadcast if they win. The details of the process looks slightly different
in *proof of burn* (PoB) and *proof of transfer* (PoX).

**In *proof of burn***, miners candidate to have their principals be the leader
of an epoch by burning tokens on the burn chain, e.g. Bitcoin. The
leader is then selected through a *verifiable random function* (VRF). The VRF
has the following properties

- The likelihood of a principal to be elected is proportional to the amount of
  tokens the principal has burned.
- The output of the VRF cannot be predicted before the burn transaction has been
  included in a block.

***Proof of transfer*** extends *Proof of burn* by instead of burning tokens to candidate,
miners transfer tokens to addresses of STX holders who participate in stacking. The
remainder of the sortition process remains the same as in PoB.

Sortition is defined in SIP-001
[here](https://github.com/stacksgov/sips/blob/main/sips/sip-001/sip-001-burn-election.md#step-3-sortition).
The PoX adaptation is defined in [SIP-007](https://github.com/stacksgov/sips/blob/506085f5da4fe5e1bee8e6387638950501361503/sips/sip-007/sip-007-stacking-consensus.md).

## What is a tenure?
A tenure is the period during which an elected leader propagates transaction data.
The tenure is terminated when a new burn chain block arrives.

For more details, see the tenure implementation in the stacks blockchain
[here](https://github.com/stacks-network/stacks-blockchain/blob/8da076e7c72f3d743caecfd12a9b32496f0a236c/testnet/stacks-node/src/tenure.rs#L26).

## What are epochs?
Epoch may refer to two things

1. The leader and leader candidate state during a block in the underlying burnchain,
as defined in [SIP-001](https://github.com/stacksgov/sips/blob/506085f5da4fe5e1bee8e6387638950501361503/sips/sip-001/sip-001-burn-election.md#definitions)

2. A significant period in the blockchain's history beginning with a hard fork. The
current epochs are defined [here](https://github.com/stacks-network/stacks-blockchain/blob/8da076e7c72f3d743caecfd12a9b32496f0a236c/stacks-common/src/types/mod.rs#L69).

## What are reward cycles?
A reward cycle is a set of epochs divided into two phases:

1. Prepare phase: The PoX anchor block and reward set is determined.
2. Reward phase: Mining any descendant of the anchor block requires burn chain tokens
                 to be transferred to members of the reward set.

Reward cycles are defined in SIP-007
[here](https://github.com/stacksgov/sips/blob/main/sips/sip-007/sip-007-stacking-consensus.md#stacking-consensus-algorithm).

## What is a PoX anchor block?
A PoX anchor block is a block on the Stacks chain which determines the reward set for
a reward cycle. During a reward cycle, mining any descendant of the anchor block requires
transferring burn chain tokens to members of the reward set.

The anchor block is determined during the prepare phase as the latest ancestor before
the phase with at least `F*w` confirmations, where `F` is a proper fraction larger than 0.5
and `w` is the number of blocks in the prepare phase. This ensures that at most one PoX
anchor block exists per reward cycle.

PoX anchor blocks are defined in SIP-007
[here](https://github.com/stacksgov/sips/blob/506085f5da4fe5e1bee8e6387638950501361503/sips/sip-007/sip-007-stacking-consensus.md#stacking-consensus-algorithm).

## What is an anchor block?
The Stacks blockchain consists for two main types of blocks,
anchor blocks and microblocks. The anchor blocks are directly
committed on the underlying burnchain through leader block commits.

An anchor block is a block in the Stacks blockchain anchored in the
underlying burnchain (Bitcoin) through a leader block commit.

The term is not used in SIP-001 but occurs in the stacks documentation
[here](https://docs.stacks.co/docs/understand-stacks/technical-specs#stacking).

## How is a PoX anchor block different from an anchor block?
A PoX anchor block is a special block in a reward cycle.
This block is also an anchor block in the sense that this block is visible on
the burnchain in a leader block commit, i.e. it is not a microblock. A reward cycle
typically contains many anchor blocks, but at most one PoX anchor block.

## What is index block hash vs block hash?

## What is a microblock?
Between anchor blocks, the leader of an epoch may stream transactions in microblocks
to enable low latency confirmations of transactions. These blocks contain transactions
and are similar to normal blockchain blocks.

To incentivise leaders of subsequent epochs
to build on top of the latest microblock rather than ignoring these and build on the latest
anchor block, a fraction of the block reward for a microblock is distributed to the leader of the next
anchor block.

Microblocks are defined in SIP-001
[here](https://github.com/stacksgov/sips/blob/506085f5da4fe5e1bee8e6387638950501361503/sips/sip-001/sip-001-burn-election.md#specification).

## What databases does stacks use?

Database file location are relative to `working_directory/mainnet`

| Database file location | Description
| --- | ---
| chainstate/estimates/fee_estimator_scalar_rate.sqlite | 
| chainstate/estimates/cost_estimator_pessimistic.sqlite | 
| chainstate/vm/index.sqlite | 
| chainstate/vm/clarity/marf.sqlite | 
| chainstate/mempool.sqlite | 
| headers.sqlite | 
| atlas.sqlite | 
| peer.sqlite | 
| burnchain/burnchain.sqlite | 
| burnchain/sortition/marf.sqlite | 

## How do I explore the sqlite db’s?

- Install sqlite:
  - Mac: brew install sqlite
- Run `sqlite3 mainnet/peer.sqlite`

## What threads does stacks-node spawn?

- Relayer thread
- Miner thread
- Chains coordinator

## What are the typical log file entries for stacks-node?

TODO: list frequency of log lines during bootstrap and during normal running conditions

## What are affirmation maps?

PoX affirmation maps

- <https://gist.github.com/jcnelson/b1aa4bef8b9adb0856b28d3a933ef9a0>
- <https://github.com/stacks-network/stacks-blockchain/pull/2707>

## Where can I learn more about Stacks internals?

Stacks 2.0 Internals <https://github.com/stacks-network/docs>

## How do I check the syntax of a Clarity smart contract?

- Install clarinet
  - From a mac: brew install clarinet
  - From source: <https://github.com/hirosystems/clarinet>
- Use clarinet to check the contract: `clarinet check src/chainstate/stacks/boot/pox-2.clar`

## What is the difference between `src/vm_clarity` and `clarity/src`?

The `clarity` directory is the Clarity VM, and it does not depend on any of the Stacks-specific data storage logic.  Instead, it defines the traits for a data storage system.  This is important, because the data store Stacks uses is bespoke and somewhat large and complex in order to efficiently deal with blockchain forks (it's described in SIP-004 if you're curious).  The `clarity_vm` directory contains the implementation for these traits that links the Clarity VM to Stacks' data store. This way, you can build clarity without building the Stacks blockchain.  This gets used to build clarinet for example.

## How do I get testnet STX coins from the faucet?

Visit https://explorer.stacks.co/sandbox/faucet?chain=testnet 

The chain=testnet will configure testnet mode and also a url for the public testnet. The Networks tab can be used to add any url for an alternate stacks network. This was done for Stacks 2.1 testing because it was a hard-fork.

## How do I stack stx from the commandline?

Pre-reqs
- a stacks account
- blockstack-cli
  - build from stacks-blockchain source
- base58 decoder
  - https://appdevtools.com/base58-encoder-decoder

Generate the transaction. stack-stx function signature from pox-2.clar.

```
(stack-stx (amount-ustx uint)
           (pox-addr (tuple (version (buff 1)) (hashbytes (buff 32))))
           (start-burn-ht uint) ; must be a burn block height inside the next reward cycle
           (lock-period uint)) ; number of reward cycles to lock for
```

note on pox-addr: version values are in pox-2.clar. p2pkh = 1 (legacy address. 20 byte hashbytes). hashbytes come from base58 decoding the bitcoin address to a 25 byte/50 char hex string and removing the leading version byte and the four trailing checksum bytes. 

```
blockstack-cli --testnet contract-call <66 byte stx private key in hex> 300 3 ST000000000000000000002AMW42H pox-2 stack-stx 
                -e u5160000000000 -e '{version: 0x01, hashbytes: 0xe8b19d771ed4f3ab18dd5c8cc6fca3a2a1c31b61}' -e u2409274 -e u1  > tx.json
```

Convert hex output from blockstack-cli to binary and post to API URL `/v2/transactions`

note: the http endpoint will ignore posts unless the content-type header is included and set for binary data.

```
cat tx.json | xxd -r -p | curl --data-binary @- -H "content-type: application/octet-stream" http://2-1-seed.testnet.hiro.so:20443/v2/transactions
```

Output is the tx id on success
"0b4774132f3252dfc242a05178691eb06c98fb5e8295917e67e501f9d44764e2"

## How do I make a call to a read-only function in a clarity contract?

A call to a read-only function in a clarity contract can be done instantly by any node and does not create a new transaction. Use the following http request. The payload specifies the stx address that is making the call to the function, and the arguments array contains strings of each serialized clarity value for each function parameter.

One way to generate the serialized clarity value is to use https://github.com/jcnelson/stacks-node-cli and the encode function.

note: the http endpoint will ignore posts unless the content-type header is included and set for json.

```
$ curl https://2-1-api.testnet.hiro.so/v2/contracts/call-read/ST000000000000000000002AMW42H/pox-2/get-total-ustx-stacked 
           -d '{"sender":"ST3MB37BQ3VAF7ARRVNE8SHQWMEHA3GRVC6QCSB7M", "arguments": ["0100000000000000000000000000000001"]}' -H "content-type: application/json"
{"okay":true,"result":"0x0100000000000000000000000000000000"}
```

## How do I get the hashbytes for a pox-addr from bitcoin compressed public key bytes?

btc public Key (compressed, 1 even/odd marker plus 32 bytes. 66 hex characters): 02048D1783065CE86FAE4B2DC67B9455EFF8B0D1F5D3BB63E9E2FFDDB4A921970D
```
$ echo 02048D1783065CE86FAE4B2DC67B9455EFF8B0D1F5D3BB63E9E2FFDDB4A921970D  | xxd -r -p | sha256sum
be6f7f08c8c8d0ff9cb47483a7d24bc984e4381c0057b7b16c315f212c46f0a0  -
$ cargo install digester --all-features
$ echo be6f7f08c8c8d0ff9cb47483a7d24bc984e4381c0057b7b16c315f212c46f0a0  | xxd -r -p |  digester -a ripemd160
e8b19d771ed4f3ab18dd5c8cc6fca3a2a1c31b61
```

## How do I Participate in the Radon 2.1 Testnet?

As announced in [Jude's tweet][TWEET_RADON], you can participate in the Radon 2.1 testnet by following these steps: mentioned in this [Gist][RADON_GISTS].

[TWEET_RADON]: https://twitter.com/judecnelson/status/1623151711530504192 
[RADON_GISTS]: https://gist.github.com/jcnelson/c5f43cbefba3727dcccf5a95209b60fc
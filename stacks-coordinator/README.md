# Coordinator for sBTC private alpha test
The stacks coordinator is a central component of the alpha sBTC testnet. The coordinator is responsible for

- Keeping track of confirmed peg-in & peg-out operations on the Bitcoin chain
- Minting & burning sBTC on the Stacks chain
- Fulfilling peg-out requests

To fulfill the above, the coordinator must:

- Communicate with signers through an http relay to sign peg-out fulfillments and set the peg-wallet address.
- Interact with the alpha sBTC contract to:
  - Set the peg-wallet address.
  - Mint & burn sBTC.
- Poll stacks-node to obtain bitcoin ops.
- Maintain an BTC & STX wallet to cover tx fees.

```mermaid
graph TD
    C[Stacks coordinator]-->|Mint/burn sBTC|D[sBTC contract]
    C-->|Set peg wallet address|D
    C-->|Get signer configuration|D
    C-->|Get sBTC ops|N[Stacks node RPC]
    C-->|DKG+Signing|P[Relay]
    S[Signer]-->|DKG+Signing|P
```

## Design draft
The coordinator is anticipated to consist of three main components for functionality:

1. A job queue which observes the state of the stacks and bitcoin blockchain. The queue can be polled for peg operations which need to be processed.
2. A fee wallet capable of creating and funding mint/burn sbtc operations on the stacks chain, and peg-out fulfillments on Bitcoin.
3. A frost coordinator, which interacts with an http relay to run frost operations such as distributed key generation and signing messages.

In addition to these three components, the coordinator will need a component `StacksNode` and a component `BitcoinNode` to facilitate interaction
with blockchain nodes.

```mermaid
classDiagram
    StacksCoordinator *-- PegQueue
    StacksCoordinator *-- FeeWallet
    StacksCoordinator *-- FrostCoordinator
    StacksCoordinator *-- StacksNode
    StacksCoordinator *-- BitcoinNode

    StacksNode <.. PegQueue
    StacksNode <.. FeeWallet
    BitcoinNode <.. FeeWallet
    
    class StacksCoordinator {
        PegQueue
        FeeWallet
        FrostCoordinator
        StacksNode
        BitcoinNode
    }

    class PegQueue {
      +next_unprocessed_peg_in_op()
      +next_unprocessed_peg_out_request_op()
    }

    class FeeWallet {
      +mint_sbtc()
      +burn_sbtc()
      +fulfill_peg_out()
      +set_wallet_address()
    }

    class FrostCoordinator {
      +run_dkg_round()
      +sign_message()
      +get_aggregate_public_key()
    }

    class StacksNode {
      +get_peg_in_ops()
      +get_peg_out_request_ops()
      +next_nonce()
      +broadcast_transaction()
    }

    class BitcoinNode {
      +broadcast_transaction()
    }
```

```rust
struct StacksCoordinator {
  job_queue: PegQueue,
  fee_wallet: FeeWallet,
  frost_coordinator: FrostCoordinator,
  stacks_node: StacksNode,
  bitcoin_node: BitcoinNode,
}

// This Queue will need a persisted state to ensure operations are read
// at most once. For the alpha test the system will guarantee at-most once porcessing
// of peg-in/peg-out requests. Any missed operations should be possible to manually handle
impl PegQueue{
  fn next_unprocessed_peg_in_op(StacksNode) -> PegInOp
  fn next_unprocessed_peg_out_request_op(StacksNode) -> PegOutRequest

  // Additional methods to ensure exactly-once processing of requests may be added
}

// Should initially focus on on mainnet stacks and mainnet bitcoin
// We may want to split this up into a BitcoinWallet and a StacksWallet depending
// on the implementation.
impl FeeWallet {
  fn mint_sbtc() -> StacksTransaction
  fn burn_sbtc() -> StacksTransaction
  fn fulfill_peg_out() -> BitcoinTransaction

  fn set_wallet_address() -> StacksTransaction
}

impl FrostCoordinator {
  fn run_dkg_round()
  fn sign_message(msg: &str) -> Signature
  fn get_aggregate_public_key() -> SignerPublicKey
}

impl StacksNode {
  fn get_peg_in_ops(...) -> [PegInOp]
  fn get_peg_out_request_ops(...) -> [PegOutRequestOp]

  fn broadcast_transaction(tx)
}

impl BitcoinNode {
  fn broadcast_transaction(tx)
}
```

## Implementation plan
These should have equivalent issues in the `core-eng` repo.

### #1: Basic structure
Initiate a new project in the `core-eng` repo with the basic structure and components envisioned for the `stacks-coordinator`.
The basic boilerplate should contain:

1. Configuration (Toml)
2. Cli structure (Clap)
3. Mocked types for external interaction points
  - Job Queue
  - Fee wallet
  - Signer coordinator
4. Event loop, incl. tests

Depends on:
- Wire formats

Time estimate: 3 days

### #2: Job queue
Logic for fetching jobs from the stacks-node RPC endpoint

Depends on:
- #1
- RPC endpoints

Time estimate: 3 days

### #3: Signer coordinator
Introduce a component to interact with the signers to run distributed key generation & sign messages.

Depends on:
- #1
- frost-signer, frost-coordinator

Time estimate: 2 days

### #4: Stacks transaction construction
Utilities to generate the necessary transactions to mint & burn sBTC.

Depends on:
- #1
- sBTC contract

Time estimate: 2 days

### #5: BTC transaction construction
Utilities to generate the bitcion transaction for peg-out fulfillments.

Depends on:
- #1

Time estimate: 2 days

## Event loop
This is a rough outline of the coordinator event loop which does not take fault-tolerance into consideration.

```mermaid
graph TD
    A[Read configuration from file and sBTC contract] --> B[Generate distributed signer key]
    B --> C{Next Bitcoin Op}
    C -->|Peg In| D[Mint sBTC]
    D -->C

    C -->|Peg Out Request| E[Burn sBTC]
    E -->F[Fulfill Peg Out]
    F -->C
```

## Feature requests
These will be added as standalone issues, not explicitly included in the implementation plan.

### Closed alpha testing
Maintain a closed list of members of the alpha testing. Only addresses in this list is allowed to peg-in & peg-out sBTC.
The list should contain Stacks addresses which are the only allowed recipients of peg-ins, and BTC addresses which are the only allowed recipients of peg-outs.

Time estimate: 1 day.

### One-off commands in CLI
The CLI can start an event loop, but it would also be nice for the coordinator to allow one-off commands such as "process THIS peg-in request" etc.

Time estimate: 1 day.

### How to Run a DKG Signing Round
In seperate terminals run the following commands:
```
relay-server $ cargo run
stacks-signer $ cargo run -- --id 3 --config conf/signer.toml
stacks-signer $ cargo run -- --id 2 --config conf/signer.toml
stacks-signer $ cargo run -- --id 1 --config conf/signer.toml
stacks-coordinator $ cargo run -- --config conf/coordinator.toml --signer-config conf/signer.toml dkg
```
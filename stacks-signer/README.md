# Stacks signer
The stacks signer is responsible for participating in the FROST signing scheme to
sign sBTC peg out fulfillments from a shared wallet.

Multiple signer instances communicate with each other and a coordinator using an HTTP relay.

The signer interacts with the sBTC contract to read public keys for other signers
and the stacks-coordniator.

```mermaid
graph TD
    S[Signer]-->|Get configuration|D[sBTC contract]
    S-->|DKG+Signing|R[Relay server]
```

## Signer design
The signer system should contain an event loop which communicates with an HTTP relay and
communicates with a signer entity.

```rust
pub trait SignerService {
    type RelayServer;
    type StacksNode; // Shared implementation with stacks-coordinator
    type Signer: Signer;

    fn relay(&self) -> &Self::RelayServer;
    fn stacks_node(&self) -> &Self::StacksNode;
    fn signer(&self) -> &Self::Signer;

    // Provided method
    fn run(mut self);
}

pub trait Signer {
  fn generate_key_shares(&mut self, round_id: u64) -> (PublicShares, PrivateShares);
  fn public_nonce(&self, round_id: u64) -> PublicNonce;
  fn signature_share(&self, round_id: u64, msg: &[u8]) -> SignatureShare;
}  
```

## Signer event loop
A rough outline of the signer event loop

```mermaid
graph TD
    A[Read configuration from file and sBTC contract] --> B{Next incoming message}
    B -->|DKG_BEGIN| C[Generate shares]
    C --> D[Send public and private shares]
    D --> B
    B -->|DKG_PUBLIC_SHARE| E[Store public share]
    E --> B
    B -->|DKG_PRIVATE_SHARE| F[Store private share]
    F --> G[Send DKG_END]
    G --> B
    B -->|NONCE_REQUEST| H[Send public nonce]
    H --> B
    B -->|SIGN_SHARE_REQUEST| I[Compute signature share]
    I --> B
```

# Relay communication charts
## Distributed key generation
```mermaid
sequenceDiagram
    participant C as Coordinator
    participant R as Relay
    participant S as Signer
    C->>+R: DKG_BEGIN
    R->>S: DKG_BEGIN
    S-->R: DKG_PUBLIC_SHARES
    S-->R: DKG_PRIVATE_SHARES
    S->>R: DKG_END
    R->>-C: DKG_END
```

## Sign message
```mermaid
sequenceDiagram
    participant C as Coordinator
    participant R as Relay
    participant S as Signer
    C->>+R: NONCE_REQUEST
    R->>S: NONCE_REQUEST
    S->>R: NONCE_RESPONSE
    R->>-C: NONCE_RESPONSE (xT)
    C->>+R: SIGN_SHARE_REQUEST
    R->>S: SIGN_SHARE_REQUEST
    S->>R: SIGN_SHARE_RESPONSE
    R->>-C: SIGN_SHARE_RESPONSE (xT)
```

## Query aggregate public key
```mermaid
sequenceDiagram
    participant C as Coordinator
    participant R as Relay
    participant S as Signer
    C->>+R: DKG_QUERY
    R->>S: DKG_QUERY
    S->>R: DKG_PUBLIC_SHARES
    R->>-C: DKG_PUBLIC_SHARES (xT)
```
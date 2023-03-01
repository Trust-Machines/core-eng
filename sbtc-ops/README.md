Deployed sBTC Alpha contract:

https://explorer.stacks.co/txid/0x4bbd04b7fc17cb832b4df78c9c06708fd2ccbe21fff5c5de8116e48fefb6126a?chain=mainnet

SCA: stacks-coordinator-alpha
SCB: stacks-coordinator-beta (mini)
SSA: stacks-signer-alpha
SSB: stacks-signer-beta (mini)

# User stories
  - User wraps BTC
    - User visits sBTC Bridge and inputs:
      - How much BTC to wrap
      - Which Stacks address (principal or contract) for sBTC token to mint
      - sBTC Bridge generates Bitcoin transaction hex payload
    - User signs Bitcoin transaction using Electrum, Trezor CLI, etc.
    - User waits for confirmation of mint
  
  - User unwraps sBTC
      - How much sBTC to un-wrap
      - Which Bitcoin address for BTC to be withdrawn to
      - sBTC Bridge generates Bitcoin transaction hex payload
    - User signs Bitcoin transaction using Electrum, Trezor CLI, etc.
    - User waits for confirmation of Bitcoin withdrawal

# Implementaion (in progress)

- sBTC Alpha
  - User wraps BTC
    - OP_RETURN (1-transaction method)
      - User crafts Bitcoin transaction using sBTC Bridge (https://trust-machines.github.io/sbtc-bridge/)
        Transaction sends funds from user's BTC wallet to the sBTC Threshold Wallet
      - SCA reads the BTC transaction from stacks-node
        SCA constructs and broadcasts a Stacks transaction to mint sBTC fungible token in Stacks chain
    
    - OP_DROP (2-transaction method)


  - User un-wraps

  - Threshold Wallet Generation
    - stacks-coordinator instructs stacks-signers (sbtc-signer) to generate a peg wallet.

- sBTC Mini

Signer signs transaction

import rpc from './rpc.mjs'

// Example:
// {"Mint":{"amount":0,"block_height":0,"burn_header_hash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"memo":[],"peg_wallet_address":{"Standard":[{"bytes":"944f997c5553a6f3e1028e707c71b5fa0dd3afa7","version":0},null]},"recipient":{"Standard":[0,[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]},"txid":"0000000000000000000000000000000000000000000000000000000000000000","vtxindex":0}}

/** @typedef {Mint|Burn} In */

/** @type {(input: In) => string} */
const f = input => input

rpc.listenStdio(f)

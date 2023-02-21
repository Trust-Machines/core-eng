import { listenStdio, toAsync, JsonMap } from './rpc.ts'
// import { makeContractCall } from 'npm:@stacks/transactions'

// Example from Rust serialization:
//  {
//      "Mint":{
//          "amount":0,
//          "block_height":0,
//          "burn_header_hash":"0000000000000000000000000000000000000000000000000000000000000000",
//          "memo":"",
//          "peg_wallet_address":"1EXCN4m6mNL88QzPwksBnpVqr5F1dC4SGa",
//          "recipient":"S0000000000000000000002AA028H",
//          "txid":"0000000000000000000000000000000000000000000000000000000000000000",
//          "vtxindex":0
//      }
//  }

type Command =
    | { readonly Mint: Mint }
    | { readonly Burn: Burn }
    | { readonly SetWalletAddress: readonly number[] }

type PoxAddress = string

type PrincipalData = string

type BurnchainHeaderHash = readonly number[]

type Mint = {
    readonly amount: number
    readonly block_height: number
    readonly burn_header_hash: BurnchainHeaderHash
    readonly memo: readonly number[]
    readonly peg_wallet_address: PoxAddress
    readonly recipient: PrincipalData
    readonly txid: string
    readonly vtxindex: number
}

type Burn = {
    readonly amount: number
    readonly block_height: number
    readonly burn_header_hash: readonly number[]
    readonly fulfillment_fee: number
    readonly memo: readonly number[]
    readonly peg_wallet_address: PoxAddress
    readonly recipient: PoxAddress
    readonly signature: string
    readonly txid: string
    readonly vtxindex: number
}

const f = (input: Command): string => {
    if ("Mint" in input) return "Mint"
    if ("Burn" in input) return "Burn"
    if ("SetWalletAddress" in input) return "SetWalletAddress"
    throw "unknown command"
}

listenStdio(toAsync(f as JsonMap))

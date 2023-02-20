import { listenStdio, JsonMap } from './rpc.ts'

// Example from Rust serialization:
//  {
//      "Mint":{
//          "amount":0,
//          "block_height":0,
//          "burn_header_hash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
//          "memo":[],
//          "peg_wallet_address":{
//              "Standard":[
//                  {"bytes":"944f997c5553a6f3e1028e707c71b5fa0dd3afa7","version":0},
//                  null
//              ]
//          },
//          "recipient":{
//              "Standard": [0,[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]
//          },
//          "txid":"0000000000000000000000000000000000000000000000000000000000000000",
//          "vtxindex":0
//      }
//  }

type Command = { readonly Mint: Mint } | { readonly Burn: Burn }

type PoxAddress = {
    readonly Standard: readonly[
        {
            readonly bytes: string
            readonly version: string
        },
        null
    ]
}

type PrincipalData = {
    readonly Standard: readonly[number, readonly number[]]
}

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

const f = (input: Command): string => 'Mint' in input ? 'Mint' : 'Burn'

listenStdio(f as JsonMap)

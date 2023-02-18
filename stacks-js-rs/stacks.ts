import { listenStdio, JsonMap, Json } from './rpc.ts'

// Example:
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

type Mint = {
    readonly amount: number,
    readonly block_height: number,
    readonly burn_header_hash: readonly number[],
    readonly memo: readonly number[],
    readonly peg_wallet_address: {
        readonly Standard: readonly[
            {
                readonly bytes: string,
                readonly version: string,
            },
            null
        ],
    },
    readonly recipient: {
        readonly Snadard: readonly[number, readonly number[]]
    },
    readonly txid: string,
    readonly vtxindex: number,
}

type Burn = Json

const f = (input: Command): string => 'Mint' in input ? 'Mint' : 'Burn'

listenStdio(f as JsonMap)

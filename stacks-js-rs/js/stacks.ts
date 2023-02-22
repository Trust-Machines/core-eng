import { listenStdio, type AsyncJsonMap } from './lib.ts'
import { AnchorMode, bufferCVFromString, makeContractCall } from 'npm:@stacks/transactions'

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

type BurnchainHeaderHash = string

type Memo = string

type Mint = {
    readonly amount: number
    readonly block_height: number
    readonly burn_header_hash: BurnchainHeaderHash
    readonly memo: Memo
    readonly peg_wallet_address: PoxAddress
    readonly recipient: PrincipalData
    readonly txid: string
    readonly vtxindex: number
}

type Burn = {
    readonly amount: number
    readonly block_height: number
    readonly burn_header_hash: BurnchainHeaderHash
    readonly fulfillment_fee: number
    readonly memo: Memo
    readonly peg_wallet_address: PoxAddress
    readonly recipient: PoxAddress
    readonly signature: string
    readonly txid: string
    readonly vtxindex: number
}

const f = async (input: Command): Promise<string> => {
    if ("Mint" in input) {
        try {
            await makeContractCall({
                contractAddress: 'SPBMRFRPPGCDE3F384WCJPK8PQJGZ8K9QKK7F59X',
                contractName: 'contract_name',
                functionName: 'contract_function',
                functionArgs: [bufferCVFromString('foo')],
                anchorMode: AnchorMode.Any,
                //
                senderKey: '0001020304050607080910111213141516171819202122232425262728293031',
            })
        } catch(e) {
            return `${e}`
        }
        return "Mint"
    }
    if ("Burn" in input) return "Burn"
    if ("SetWalletAddress" in input) return "SetWalletAddress"
    throw "unknown command"
}

listenStdio(f as AsyncJsonMap)

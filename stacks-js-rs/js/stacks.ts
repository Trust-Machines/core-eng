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
    | { readonly SetWalletAddress: SetWalletAddress }

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

type SetWalletAddress = readonly number[]

type MintCommand = (input: Mint) => Promise<string>
type BurnCommand = (input: Burn) => Promise<string>
type SetWalletAddressCommand = (input: SetWalletAddress) => Promise<string>

const mint: MintCommand = async (): Promise<string> => {
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
    } catch (e) {
        return `${e}`
    }
    return "Mint"
}

const burn: BurnCommand = async (): Promise<string> =>
    await "Burn"

const set_wallet_address: SetWalletAddressCommand = async (): Promise<string> =>
    await "SetWalletAddress"

const f = (input: Command): Promise<string> => {
    if ("Mint" in input) return mint(input.Mint)
    if ("Burn" in input) return burn(input.Burn)
    if ("SetWalletAddress" in input) return set_wallet_address(input.SetWalletAddress)
    throw "unknown command"
}

listenStdio(f as AsyncJsonMap)

import { listenStdio, dispatch, CommandMap } from '../lib.ts'
import {
    type SignedContractCallOptions,
    makeContractCall,
} from 'npm:@stacks/transactions'

type MakeContractCallInput = {
    readonly[k in keyof SignedContractCallOptions]:
        k extends 'functionArgs' ? readonly string[] : SignedContractCallOptions[k]
}

const t = {
    makeContractCall: (input: MakeContractCallInput) => {
        const o = { ...input, functionArgs: [] }
        return makeContractCall(o)
    }
}

listenStdio(dispatch(t as unknown as CommandMap))

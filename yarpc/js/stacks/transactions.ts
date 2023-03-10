import { listenStdio, dispatch, CommandMap } from '../lib.ts'
import {
    type SignedContractCallOptions,
    makeContractCall,
    deserializeCV,
} from 'npm:@stacks/transactions'

type MakeContractCallInput = {
    readonly[k in keyof SignedContractCallOptions]:
        k extends 'functionArgs' ? readonly string[] : SignedContractCallOptions[k]
}

const t = {
    makeContractCall: (input: MakeContractCallInput) =>
        makeContractCall({ ...input, functionArgs: input.functionArgs.map(deserializeCV) })
}

listenStdio(dispatch(t as unknown as CommandMap))

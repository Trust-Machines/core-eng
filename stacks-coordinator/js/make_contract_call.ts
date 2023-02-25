import { listenStdio, type AsyncJsonMap } from '../../yarpc/js/lib.ts'
import { makeContractCall } from 'npm:@stacks/transactions'

listenStdio(makeContractCall as unknown as AsyncJsonMap)

import { listenStdio, dispatch, CommandMap } from '../lib.ts'
import * as transactions from 'npm:@stacks/transactions'

listenStdio(dispatch(transactions as unknown as CommandMap))

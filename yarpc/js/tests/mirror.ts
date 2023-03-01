import { listenStdio, toAsync } from '../lib.ts'

listenStdio(toAsync(v => v))

import { listenStdio, toAsync } from '../../yarpc/js/lib.ts'

listenStdio(toAsync(v => { throw v }))

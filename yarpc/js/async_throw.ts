import { listenStdio } from '../../yarpc/js/lib.ts'

listenStdio(v => Promise.reject(v))

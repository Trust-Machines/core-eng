import { listenStdio } from '../lib.ts'

listenStdio(v => Promise.reject(v))

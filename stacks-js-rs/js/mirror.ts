import { listenStdio, toAsync } from "./rpc.ts"

listenStdio(toAsync(v => [v]))

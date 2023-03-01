import { stdin, stdout } from 'node:process'

export type JsonObject = {
    readonly [k in string]: Json
}

export type JsonArray = readonly Json[]

export type Json = JsonObject | boolean | string | number | null | JsonArray

const { parse, stringify: JsonStringify } = JSON

const stringify = (s: Json) => JsonStringify(s, (_, value) =>
    typeof value === 'bigint'
        ? value.toString()
        : value
)

type Ok = { Ok: Json }

type Err = { Err: string }

/// This type is compatible with Rust `serde` serialization of `std::Result`.
type Result = Ok | Err

const writeResult = (result: Result) => stdout.write(`${stringify(result)}\n`)

const writeError = (e: unknown) => writeResult({ Err: `lib: ${e}` })

/// Writes an error into STDIO if the f throws an exception.
const tryCatch = (f: () => void) => {
    try {
        f()
    } catch (e) {
        writeError(e)
    }
}

const writeOk = (ok: Json) => tryCatch(() => writeResult({ Ok: ok }))

export type JsonMap = (input: Json) => Json

export type AsyncJsonMap = (input: Json) => Promise<Json>

export const listenStdio = (f: AsyncJsonMap) => {
    let buffer = ""
    stdin.setEncoding('utf8').on('readable', () => {
        for (; ;) {
            const x: string = stdin.read()
            if (x === null) { break }
            const i = x.indexOf('\n')
            if (i === -1) {
                buffer += x
            } else {
                const input = buffer + x.substring(0, i)
                buffer = x.substring(i + 1)
                tryCatch(() => f(parse(input)).then(writeOk).catch(writeError))
            }
        }
    })
}

export type CommandMap = { readonly [k in string]: AsyncJsonMap }

export type DispatchCommand = readonly [string, Json];

export const dispatch = (map: CommandMap): AsyncJsonMap =>
    (([command, arg]: DispatchCommand) => map[command](arg)) as AsyncJsonMap

export const toAsync = (f: JsonMap): AsyncJsonMap => v => Promise.resolve(f(v))

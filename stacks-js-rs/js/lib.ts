import { stdin, stdout } from "node:process"

type JsonObject = {
    readonly [k in string]: Json
}

type JsonArray = readonly Json[]

export type Json = JsonObject | boolean | string | number | null | JsonArray

type GlobalJson = {
    readonly parse: (v: string) => Json
    readonly stringify: (v: Json) => string
}

const { parse, stringify }: GlobalJson = JSON

type Ok = { Ok: Json }

type Err = { Err: string }

type Result = Ok | Err

const writeResult = (result: Result) => stdout.write(`${stringify(result)}\n`)

const writeError = (e: unknown) => writeResult({ Err: `${e}` })

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
    stdin.setEncoding("utf8").on("readable", () => {
        for (; ;) {
            const x: string = stdin.read()
            if (x === null) { break }
            const i = x.indexOf("\n")
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

export const toAsync = (f: JsonMap): AsyncJsonMap => v => Promise.resolve(f(v))

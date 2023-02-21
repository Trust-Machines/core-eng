import { stderr, stdin, stdout } from "node:process"

type JsonObject = {
    readonly [k in string]: Json
}

type JsonArray = readonly Json[]

export type Json = JsonObject | boolean | string | number | null | JsonArray

type Ok<T> = readonly ["ok", T]

type Error<E> = readonly ["error", E]

type Result<T, E> = Ok<T> | Error<E>

const json_try_parse = (input: string): Result<Json, "invalid JSON"> => {
    try {
        return ["ok", JSON.parse(input)]
    } catch (_) {
        return ["error", "invalid JSON"]
    }
}

export type JsonMap = (input: Json) => Json

export const listenStdio = (f: JsonMap) => {
    /** @type {string} */
    let buffer = ""
    stdin.setEncoding("utf8").on("readable", () => {
        for (; ;) {
            /** @type {string|null} */
            const x = stdin.read()
            if (x === null) { break }
            const p = x.indexOf("\n")
            if (p === -1) {
                buffer += x
            } else {
                const input = buffer + x.substring(0, p)
                buffer = x.substring(p + 1)
                const [t, v] = json_try_parse(input)
                if (t === "ok") {
                    stdout.write(JSON.stringify(f(v)))
                    stdout.write("\n")
                } else {
                    stderr.write(`error: ${v}\n`)
                }
            }
        }
    })
}

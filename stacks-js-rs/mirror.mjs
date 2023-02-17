import { stdin, stdout, stderr } from 'node:process'
import stacksConnect from 'npm:@stacks/connect'

/**
 * @typedef {{
*  readonly [k in string]: Json
* }} JsonObject
*/

/** @typedef {Json[]} JsonArray */

/** @typedef {JsonObject|boolean|string|number|null|JsonArray} Json */

/**
 * @template T
 * @typedef {readonly["ok", T]} Ok
 */

/**
 * @template E
 * @typedef {readonly["error", E]} Error
 */

/**
 * @template T,E
 * @typedef {Ok<T>|Error<E>} Result
 */

/** @type {(input: string) => Result<Json, "invalid JSON">} */
const json_try_parse = input => {
    try {
        return ['ok', JSON.parse(input)]
    } catch (_) {
        return ['error', 'invalid JSON']
    }
}

/** @typedef {(input: Json) => Json} JsonMap */

/** @type {(f: JsonMap) => void} */
const listenStdio = f => {
    /** @type {string} */
    let buffer = ""
    stdin.setEncoding('utf8').on('readable', () => {
        while (true) {
            /** @type {string|null} */
            const x = stdin.read()
            if (x === null) { break }
            const p = x.indexOf('\n')
            if (p === -1) {
                buffer += x
            } else {
                const input = buffer + x.substring(0, p)
                buffer = x.substring(p + 1)
                const [t, v] = json_try_parse(input)
                if (t === 'ok') {
                    stdout.write(JSON.stringify(f(v)))
                    stdout.write('\n')
                } else {
                    stderr.write(`error: ${v}\n`)
                }
            }
        }
    })
}

listenStdio(v => [v])

import { stdin, stdout, stderr } from 'node:process'
import stacksConnect from 'npm:@stacks/connect'

/**
 * @typedef {{
*  [k in string]: Json
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

/** @type {(v: Json) => Result<Json, string>} */
const call = v => {
    switch (typeof v) {
        case 'boolean': return ['boolean', v]
        case 'number': return ['number', v]
        case 'string': return ['string', v]
        default: {
            if (v === null) { return ['null'] }
            if (v instanceof Array) { return ['array', v] }
            return ['object', v]
        }
    }
}

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
                stdout.write(JSON.stringify(call(v)))
                stdout.write('\n')
            } else {
                stderr.write(`error: ${v}\n`)                
            }                        
        }
    }
})

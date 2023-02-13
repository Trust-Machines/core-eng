import { stdin, stdout } from 'node:process'
import stacksConnect from 'npm:@stacks/connect'

/**
 * @typedef {{
*  [k in string]: Json
* }} JsonObject
*/

/** @typedef {Json[]} JsonArray */

/** @typedef {JsonObject|boolean|string|number|null|JsonArray} Json */

/** @type {(v: Json) => Json} */
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

let len = 0
/** @type {string} */
let buffer = ""

const process = () => {
    while (true) {
        if (len === 0) {
            const p = buffer.indexOf('|');
            if (p === -1) {
                break
            }
            len = parseInt(buffer.substring(0, p))
            buffer = buffer.substring(p + 1)
        } else {
            if (buffer.length < len) {
                break
            }
            const result = JSON.stringify(call(JSON.parse(buffer.substring(0, len))))
            stdout.write(`${result.length}|${result}`)
            buffer = buffer.substring(len)
            len = 0
        }
    }
}

stdin.setEncoding('utf8').on('readable', () => {
    while (true) {
        const x = stdin.read()
        if (x === null) { break }
        buffer += x
        process()
    }
})

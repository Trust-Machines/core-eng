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
            let output
            try {
                output = ['ok', call(JSON.parse(input))]
            } catch (e) {
                output = ['err', e]
            }
            stdout.write(JSON.stringify(output))
            stdout.write('\n')
        }
    }
})

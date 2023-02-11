import { stdin, stdout } from 'node:process'

/**
 * @typedef {{
*  [k in string]: Unknown
* }} Object
*/

/** @typedef {Unknown[]} Array */

/** @typedef {Object|boolean|string|number|null|Array} Unknown */

/** @type {(v: Unknown) => Unknown} */
const call = v => {
    switch (typeof v) {
        case "boolean": return ["boolean", v]
        case "number": return ["object", v]
        case "string": return ["string", v]
        default: {
            if (v === null) { return ["null"] }
            if (v instanceof Array) { return ["array", v] }
            return ["object", v]
        }
    }
}

let len = 0
/** @type {string} */
let buffer = ""

const process = () => {
    while (true) {
        if (len === 0) {
            const p = buffer.indexOf("|");
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
            stdout.write(result.length + '|' + result)
            buffer = buffer.substring(len)
            len = 0
        }
    }
}

stdin.setEncoding("utf8")

stdin.on('readable', () => {
    while (true) {
        const x = stdin.read()
        if (x === null) { break }
        buffer += x
    }
    process()
})

// while(true) {}
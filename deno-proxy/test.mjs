import { stdin, stdout } from 'node:process'

/** @type {(v: string) => void} */
const call = v => {
    const s = JSON.stringify({v});
    stdout.write(s.length + '|' + s)
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
            call(buffer.substring(0, len))
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
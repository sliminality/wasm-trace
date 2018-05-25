// @format

const evaluateWasm = require('../bin/runWasm').default;
const BUFFER_SIZE = 10;

evaluateWasm(process.argv)
  .then(({ result, exports }) => {
    const pointer = result;
    const buffer = new Int32Array(exports.memory.buffer, pointer, BUFFER_SIZE);
    console.log('Buffer contents:', buffer);
    return result;
  })
  .then(console.log, console.error);

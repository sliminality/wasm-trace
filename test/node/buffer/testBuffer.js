// @format

const { compileAndRun, readMemory } = require('../bin/runWasm');
const BUFFER_SIZE = 10;

compileAndRun(process.argv)
  .then(({ result, exports }) => {
    const pointer = result;
    readMemory(exports.memory, pointer, BUFFER_SIZE);
    return result;
  })
  .then(console.log, console.error);

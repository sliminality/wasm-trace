// @format

const { default: main } = require('../js/runWasm');
const { getMemory } = require('../js/tracer.js');
const BUFFER_SIZE = 10;

main(process.argv)
  .then(({ result, exports }) => {
    const pointer = result;
    console.log(getMemory(exports.memory, pointer, BUFFER_SIZE));
    return result;
  })
  .then(console.log, console.error);

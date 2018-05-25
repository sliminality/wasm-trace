// @format

// Execute a .wasm file in Node.js.
// `node runWasm.js arithmetic.wasm add1 10`
//
// Based on:
// https://gist.github.com/kanaka/3c9caf38bc4da2ecec38f41ba24b77df
// https://gist.github.com/kanaka/3c9caf38bc4da2ecec38f41ba24b77df#gistcomment-2564224

const fs = require('fs');
const assert = require('assert');

assert('WebAssembly' in global, 'WebAssembly global object not detected');
assert(
  process.argv.length >= 4,
  'Usage: ./runwasm.js prog.wasm func INT_ARG...',
);

const [_, __, wasmFile, funcName, ...args] = process.argv;
const bytes = fs.readFileSync(wasmFile);

WebAssembly.compile(bytes)
  .then(
    module =>
      new WebAssembly.Instance(module, {
        env: {
          memory: new WebAssembly.Memory({ initial: 256 }),
          table: new WebAssembly.Table({ initial: 0, element: 'anyfunc' }),
        },
      }),
  )
  .then(instance => {
    const { exports } = instance;
    assert(exports, 'no exports found');
    assert(funcName in exports, `${funcName} not found in wasm exports`);
    return exports[funcName](...args);
  })
  .then(console.log, console.error);

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

// Compile and run a WebAssembly module, given a path.
function compileAndRun(bytes, func, ...args) {
  return WebAssembly.compile(bytes)
    .then(
      module =>
        new WebAssembly.Instance(module, {
          memory: new WebAssembly.Memory({ initial: 256 }),
          table: new WebAssembly.Table({ initial: 0, element: 'anyfunc' }),
        }),
    )
    .then(instance => {
      const { exports } = instance;
      assert(exports, 'no exports found');
      assert(
        func in exports,
        `${func} not found in wasm exports: ${Object.keys(exports)}`,
      );
      const result = exports[func](...args);
      return { result, exports };
    });
}

// Print the contents of a memory region to the console.
function readMemory(memory, offset, length = 1) {
  const buffer = new Int32Array(memory.buffer, offset, length);
  console.log(buffer);
}

function validateArgs(_, __, wasmFile, funcName, args) {
  assert(wasmFile && funcName, 'Usage: ./runwasm.js prog.wasm func INT_ARG...');
  const parsedArgs = args.split(' ').map(x => parseInt(x, 10));
  return [wasmFile, funcName, ...parsedArgs];
}

function main(argv) {
  const [wasmFile, funcName, ...args] = validateArgs(...argv);
  const bytes = fs.readFileSync(wasmFile);
  return compileAndRun(bytes, funcName, ...args);
}

if (module.parent) {
  // Module is being imported, rather than invoked standalone.
  module.exports = {
    compileAndRun,
    readMemory,
  };
  module.exports.default = main;
} else {
  // Script is invoked from the terminal, compile and log result.
  main(process.argv)
    .then(({ result }) => result)
    .then(console.log, console.error);
}

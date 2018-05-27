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

function validateArgs(_, __, wasmFile, funcName, args) {
  assert(wasmFile && funcName, 'Usage: ./runwasm.js prog.wasm func INT_ARG...');
  const parsedArgs = args.split(' ').map(x => parseInt(x, 10));
  return [wasmFile, funcName, ...parsedArgs];
}

function compileAndRun(argv) {
  const [wasmFile, funcName, ...args] = validateArgs(...argv);
  const bytes = fs.readFileSync(wasmFile);
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
        funcName in exports,
        `${funcName} not found in wasm exports: ${Object.keys(exports)}`,
      );
      const result = exports[funcName](...args);
      return { result, exports };
    });
}

if (module.parent) {
  // Module is being imported, rather than invoked standalone.
  module.exports.default = compileAndRun;
} else {
  // Script is invoked from the terminal, compile and log result.
  compileAndRun(process.argv)
    .then(({ result }) => result)
    .then(console.log, console.error);
}

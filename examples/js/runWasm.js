// @format

// Execute a .wasm file in Node.js.
// `node runWasm.js arithmetic.wasm add1 10`

const fs = require('fs');
const assert = require('assert');
const { getNames } = require('./names.js');
const { readBuffer } = require('./tracer.js');

assert('WebAssembly' in global, 'WebAssembly global object not detected');

// Compile and run a WebAssembly module, given a path.
// Based on:
// https://gist.github.com/kanaka/3c9caf38bc4da2ecec38f41ba24b77df
// https://gist.github.com/kanaka/3c9caf38bc4da2ecec38f41ba24b77df#gistcomment-2564224
function compileAndRun(bytes, func, ...args) {
  return WebAssembly.compile(bytes)
    .then(module => {
      const instance = new WebAssembly.Instance(module, {
        memory: new WebAssembly.Memory({ initial: 256 }),
        table: new WebAssembly.Table({ initial: 0, element: 'anyfunc' }),
      });
      const names = getNames(module);
      return { instance, names };
    })
    .then(({ instance, names }) => {
      const { exports } = instance;
      assert(exports, 'no exports found');
      assert(
        func in exports,
        `${func} not found in wasm exports: ${Object.keys(exports)}`,
      );

      console.log(
        'Invoking exported function',
        func,
        'with arguments',
        args,
        '...',
      );

      const result = exports[func](...args);

      return { result, exports, names };
    });
}

function validateArgs(_, __, wasmFile, funcName, args) {
  assert(wasmFile && funcName, 'Usage: ./runwasm.js prog.wasm func INT_ARG...');
  const parsedArgs = args.split(' ').map(x => parseInt(x, 10));
  return [wasmFile, funcName, ...parsedArgs];
}

function main(argv) {
  const [wasmFile, funcName, ...args] = validateArgs(...argv);
  const bytes = fs.readFileSync(wasmFile);

  return compileAndRun(bytes, funcName, ...args)
    .then(({ result, exports, names }) => {
      console.log('Result of function call:', result);
      readBuffer(exports, names);
      return { result, exports, names };
    })
    .catch(console.error);
}

if (module.parent) {
  // Module is being imported, rather than invoked standalone.
  module.exports = {
    compileAndRun,
  };
  module.exports.default = main;
} else {
  // Script is invoked from the terminal, compile and log result.
  main(process.argv).catch(console.error);
}

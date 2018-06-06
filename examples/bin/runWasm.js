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

// Naming for tracer methods.
const TRACER = {
  LOG_CALL: '__log_call',
  EXPOSE_TRACER: '__expose_tracer',
  EXPOSE_TRACER_LEN: '__expose_tracer_len',
};

const ENTRY_KIND = {
  FUNCTION_CALL: 0,
  FUNCTION_RETURN_VOID: 1,
  FUNCTION_RETURN_VALUE: 2,
};

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
      console.log(
        'Invoking exported function',
        func,
        'with arguments',
        args,
        '...',
      );
      return { result, exports };
    });
}

function getMemory(memory, offset, length = 1) {
  return new Int32Array(memory.buffer, offset, length);
}

function chunk(size, arr = []) {
  const result = [];
  let nextChunk = [];
  for (let i = 0; i < arr.length; i += 1) {
    if (i > 0 && i % size === 0) {
      result.push(nextChunk);
      nextChunk = [];
    }
    nextChunk.push(arr[i]);
  }
  if (nextChunk.length) {
    result.push(nextChunk);
  }
  return result;
}

assert.deepStrictEqual(chunk(2, [1, 2, 'a', 'z', true, false, 1]), [
  [1, 2],
  ['a', 'z'],
  [true, false],
  [1],
]);

// Print the contents of the tracer buffer, if available.
function readBuffer(exports) {
  if (exports[TRACER.EXPOSE_TRACER] && exports[TRACER.EXPOSE_TRACER_LEN]) {
    const tracer = exports[TRACER.EXPOSE_TRACER]();
    const len = exports[TRACER.EXPOSE_TRACER_LEN]();
    const callBuffer = getMemory(exports.memory, tracer, len);
    const stack = [];

    const indent = () => '  | '.repeat(stack.length);

    for (const [kind, data] of chunk(2, callBuffer)) {
      if (kind === ENTRY_KIND.FUNCTION_CALL) {
        console.log(indent(), 'call function', data);
        stack.push(data);
      } else {
        const callee = stack.pop();
        const value = kind === ENTRY_KIND.FUNCTION_RETURN_VALUE ? [data] : [];
        console.log(indent(), 'return', ...value, 'from', callee);
      }
    }
  }
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
    getMemory,
  };
  module.exports.default = main;
} else {
  // Script is invoked from the terminal, compile and log result.
  main(process.argv)
    .then(({ result, exports }) => {
      console.log('Result of function call:', result);
      readBuffer(exports);
    })
    .catch(console.error);
}

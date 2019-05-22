// Execute a .wasm file in Node.js.
// `node runWasm.js arithmetic.wasm add1 10`
//
// Based on:
// https://gist.github.com/kanaka/3c9caf38bc4da2ecec38f41ba24b77df
// https://gist.github.com/kanaka/3c9caf38bc4da2ecec38f41ba24b77df#gistcomment-2564224

const assert = require('assert');
const { getNames } = require('./names.js');

const createInstance = (module) => {
    const config = {
        memory: new WebAssembly.Memory({
            initial: 256,
        }),
        table: new WebAssembly.Table({
            initial: 0,
            element: 'anyfunc',
        }),
    };
    return {
        instance: new WebAssembly.Instance(module, config),
        names: getNames(module),
    };
};

const invokeFunction = (func, ...args) => ({ instance, names }) => {
    const { exports } = instance;
    assert(exports, 'no exports found');

    const exportedFunction = exports[func];
    assert(
        exportedFunction,
        `${func} not found in wasm exports: ${Object.keys(exports)}`,
    );

    console.log(
        'Invoking exported function',
        func,
        'with arguments',
        args,
        '...',
    );

    return {
        names,
        exports,
        result: exportedFunction(...args),
    };
};

module.exports = { createInstance, invokeFunction };

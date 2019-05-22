// @format
const fs = require('fs');
const assert = require('assert');
const { createInstance, invokeFunction } = require('./compile.js');
const { readBuffer } = require('./tracer.js');

const validateArgs = (_, __, wasmFile, funcName, args) => {
    assert(
        wasmFile && funcName,
        'Usage: ./runwasm.js prog.wasm func INT_ARG...',
    );

    const parsedArgs = args
        .split(' ')
        .map(x => parseInt(x, 10));

    return [wasmFile, funcName, ...parsedArgs];
};

const logResult = ({ result, exports, names }) => {
    console.log('Result of function call:', result);

    // Print the contents of the ring buffer.
    readBuffer(exports, names);

    return { result, exports, names };
};

assert('WebAssembly' in global, 'WebAssembly global object not detected');

function main(argv) {
    const [wasmFile, funcName, ...args] = validateArgs(...argv);
    const bytes = fs.readFileSync(wasmFile);

    return WebAssembly.compile(bytes)
        .then(createInstance)
        .then(invokeFunction(funcName, ...args))
        .then(logResult)
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

// @format
const { chunk } = require('./util.js');

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

function getMemory(memory, offset, length = 1) {
    return new Int32Array(memory.buffer, offset, length);
}

// Print the contents of the tracer buffer, if available.
function readBuffer(exports, nameMap = new Map()) {
    const getTracerOffset = exports[TRACER.EXPOSE_TRACER];
    const getTracerLen = exports[TRACER.EXPOSE_TRACER_LEN];
    const hasDeps = getTracerOffset && getTracerLen;

    if (!hasDeps) {
        console.error('Could not find tracer dependencies in module.');
        return;
    }

    console.log('\nExecution trace:');

    const tracer = getTracerOffset();
    const len = getTracerLen();

    // Read the buffer in two-byte chunks.
    const callBuffer = getMemory(exports.memory, tracer, len);
    const chunks = chunk(2, callBuffer);

    const stack = [];
    const indent = () => '  | '.repeat(stack.length);

    for (const [kind, data] of chunks) {
        if (kind === ENTRY_KIND.FUNCTION_CALL) {
            const callee = nameMap.has(data)
                ? nameMap.get(data)
                : data;

            console.log(indent(), 'call function', callee);

            // Push the called function onto the local stack.
            stack.push(data);
        } else {
            const callee = stack.pop();
            const calleeFormat = nameMap.has(callee)
                ? nameMap.get(callee)
                : callee;

            const value = kind === ENTRY_KIND.FUNCTION_RETURN_VALUE
                ? [data]
                : [];

            console.log(
                indent(),
                'return',
                ...value,
                'from',
                calleeFormat,
            );
        }
    }
}

module.exports = { readBuffer, getMemory };

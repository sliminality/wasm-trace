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
  console.log('\nExecution trace:');
  if (exports[TRACER.EXPOSE_TRACER] && exports[TRACER.EXPOSE_TRACER_LEN]) {
    const tracer = exports[TRACER.EXPOSE_TRACER]();
    const len = exports[TRACER.EXPOSE_TRACER_LEN]();

    const callBuffer = getMemory(exports.memory, tracer, len);
    const stack = [];
    const indent = () => '  | '.repeat(stack.length);

    for (const [kind, data] of chunk(2, callBuffer)) {
      if (kind === ENTRY_KIND.FUNCTION_CALL) {
        const callee = nameMap.has(data) ? nameMap.get(data) : data;
        console.log(indent(), 'call function', callee);
        stack.push(data);
      } else {
        const callee = stack.pop();
        const calleeFormat = nameMap.has(callee) ? nameMap.get(callee) : callee;
        const value = kind === ENTRY_KIND.FUNCTION_RETURN_VALUE ? [data] : [];
        console.log(indent(), 'return', ...value, 'from', calleeFormat);
      }
    }
  }
}

module.exports = { readBuffer, getMemory };

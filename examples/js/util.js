// @format
const assert = require('assert');

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

module.exports = { chunk };

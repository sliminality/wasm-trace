// @format
// Functionality for parsing the names section of a WebAssembly Module.

const assert = require('assert');
const { TextDecoder } = require('util');

// TODO: Clean this up.
function getNames(module) {
  const nameSections = WebAssembly.Module.customSections(module, 'name');
  if (nameSections.length === 0) {
    console.log('No name sections provided');
    return;
  }
  const [names] = nameSections;
  const data = new DataView(names);
  const decoder = new TextDecoder('utf-8');

  const PAYLOAD_SIZE_OFFSET = 1;
  const { results, bytesRead: metadataSize } = readVarUint32(
    data,
    PAYLOAD_SIZE_OFFSET,
    2,
  );
  const [payloadLen, entryCount] = results;

  let entryId = 0;
  let entryOffset = PAYLOAD_SIZE_OFFSET + metadataSize;
  const nameMap = new Map();

  while (entryId < entryCount) {
    const { results, bytesRead } = readVarUint32(data, entryOffset, 2);
    const [namingId, nameSize] = results;
    assert.equal(namingId, entryId, 'Correctly identified entry id');

    const nameOffset = entryOffset + bytesRead;
    const name = decoder.decode(new Uint8Array(names, nameOffset, nameSize));
    nameMap.set(namingId, name);

    entryOffset = nameOffset + nameSize;
    entryId += 1;
  }

  return nameMap;
}

// TODO: Wrap this in a generator.
function readVarUint32(view, offset = 0, intsToRead = 1) {
  const results = [];
  let bytesRead = 0;

  for (let i = 0; i < intsToRead; i += 1) {
    let value = 0;
    let j = 0;

    while (true) {
      const byte = view.getUint8(offset + bytesRead);
      const shift = j * 7;
      value |= (byte & 0x7f) << shift;
      bytesRead += 1;
      j += 1;

      // Last byte (in little endian) starts with a 0.
      if ((byte & 0x80) === 0) {
        break;
      }
    }

    results.push(value);
  }

  return { results, bytesRead };
}

module.exports = { getNames };

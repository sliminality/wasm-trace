# wasm-trace

> a tool that would take a wasm module and modify its code to inject tracing calls, so that you could get an trace of the wasm's execution in the console

Based on an [idea](https://gist.github.com/fitzgen/34073d61f2c358f2b35038fa263b74a3) by [Nick Fitzgerald](https://github.com/fitzgen) from Mozilla.

## Requirements

The following programs must be installed to run the tests in `tests/`:

- [wasm-gc](https://github.com/alexcrichton/wasm-gc), which removes
unneeded exports, imports, and functions.
- [Binaryen](https://github.com/WebAssembly/binaryen/), a compiler
toolchain for WebAssembly. In particular, we're using the `wasm-dis`
tool to disassemble a `.wasm` binary into the readable `.wat` S-expression format.
- [Node.js](https://nodejs.org/) with WebAssembly support.

## Team

[Meg Grasse](http://github.com/meggrasse) and [Sarah Lim](http://github.com/sarahlim), with support from [Jim Blandy](https://github.com/jimblandy) and [Nick Fitzgerald](https://github.com/fitzgen).

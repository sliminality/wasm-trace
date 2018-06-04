# wasm-trace

> a tool that would take a wasm module and modify its code to inject tracing calls, so that you could get an trace of the wasm's execution in the console

Based on an [idea](https://gist.github.com/fitzgen/34073d61f2c358f2b35038fa263b74a3) by [Nick Fitzgerald](https://github.com/fitzgen) from Mozilla.

## Current status

List the instructions in each module function:

```sh
> cd ~/git/wasm-trace/examples/function-calls
> make
# node ../bin/runWasm.js function-calls.wasm double_subtract5_add1 10
Invoking exported function double_subtract5_add1 with arguments [ 10 ] ...
Result of function call: 16
Calls: Int32Array [  ]

# cargo run function-calls.wasm
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `/Users/sarah/git/wasm-trace/target/debug/wasm-trace function-calls.wasm`
Modified wasm module -> output.wasm

# node ../bin/runWasm.js output.wasm double_subtract5_add1 10

Invoking exported function double_subtract5_add1 with arguments [ 10 ] ...
Result of function call: 16
Calls: Int32Array [ 3, 4, 4, 4, 5 ]  # indices of called functions
```

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

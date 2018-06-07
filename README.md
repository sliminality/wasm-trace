# wasm-trace

> a tool that would take a wasm module and modify its code to inject tracing calls, so that you could get an trace of the wasm's execution in the console

Based on an [idea](https://gist.github.com/fitzgen/34073d61f2c358f2b35038fa263b74a3) by [Nick Fitzgerald](https://github.com/fitzgen) from Mozilla.

## Usage

Given the following Rust program:

```rust
#[macro_use]
extern crate wasm_trace;

use wasm_trace::tracer::Tracer;

tracer_dependencies!();
tracer_bootstrap!();

#[no_mangle]
pub extern "C" fn do_stuff(x: i32) -> i32 {
    println!("{}", double(x) + double(x));
    println!("{}", factorial(x as u32));
    let result = double(x) + negate(5) + 1;
    void();
    return result;
}

#[no_mangle]
pub fn double(x: i32) -> i32 {
    return x * 2;
}

#[no_mangle]
pub fn negate(x: i32) -> i32 {
    return -1 * x;
}

#[no_mangle]
pub fn void() {
    println!("No return value here!");
}

#[no_mangle]
pub fn factorial(n: u32) -> u32 {
    if n == 1 || n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
```

We can compile this program to a `.wasm` binary and pass that binary to our program:
```sh
> cargo build --example function-calls --target=wasm32-unknown-unknown
> cp target/wasm32-unknown-unknown/debug/examples/function-calls.wasm .
> cargo run function-calls.wasm  # `cargo run` our `wasm-trace` binary
```

This will output an instrumented binary called `output.wasm`. We can evaluate this module in Node.js and invoke `do_stuff(4)`:
```sh
> node examples/js/runWasm.js function-calls.wasm do_stuff 4
Invoking exported function do_stuff with arguments [ 4 ] ...
Result of function call: 4
 call function do_stuff
  |  call function double
  |  return 8 from double
  |  call function double
  |  return 8 from double
  |  call function factorial
  |   |  call function factorial
  |   |   |  call function factorial
  |   |   |   |  call function factorial
  |   |   |   |  return 1 from factorial
  |   |   |  return 2 from factorial
  |   |  return 6 from factorial
  |  return 24 from factorial
  |  call function double
  |  return 8 from double
  |  call function negate
  |  return -5 from negate
  |  call function void
  |  return from void
 return 4 from do_stuff
```

## Requirements

The following programs must be installed to run the tests in `tests/`:

- [wasm-gc](https://github.com/alexcrichton/wasm-gc), which removes
unneeded exports, imports, and functions.
- [Binaryen](https://github.com/WebAssembly/binaryen/), a compiler
toolchain for WebAssembly. In particular, we're using the `wasm-dis`
tool to disassemble a `.wasm` binary into the readable `.wat` S-expression format.
- [Node.js](https://nodejs.org/) with WebAssembly support.

## Current functionality

Goal: P1 functionality by project handin.

- Instrument functions
  - [x] (P0) Instrument exported functions
  - [ ] (P1) Instrument the whole function section, including standard library functions, *except* for the transitive closure of the functions called by the tracer
  - [ ] Filter function names by regex
- Calls
  - [x] (P0) Log that a call happened
  - [ ] (P2) Log arguments to calls
  - [ ] (P3) Capture call frequency
- Returns
  - [x] (P0) Log that a return happened
  - [x] (P1) Log returned value if `i32`
  - [ ] (P1) Log returned value if wasm-supported type
    - Instrumentation is done, but need to update JavaScript and tracer to support 64 bits.
  - [ ] (P2) Log returned value if Rust type (via pointer)
- UI/UX
  - [x] (P0) Display function ids
  - [x] (P1) Display formatted names
  - [x] (P0) Bootstrap the tracer using macros
  - [ ] (P2) Bootstrap the tracer using `extern crate wasm_trace;` alone
  - [ ] (P2) Allow users to specify ring buffer size
  - [ ] (P3) Display histogram of call frequency

## Team

[Meg Grasse](http://github.com/meggrasse) and [Sarah Lim](http://github.com/sarahlim), with support from [Jim Blandy](https://github.com/jimblandy) and [Nick Fitzgerald](https://github.com/fitzgen).

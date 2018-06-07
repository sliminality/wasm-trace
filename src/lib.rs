#[macro_use]
mod macros;
mod either;
/// WebAssembly module and components.
pub mod module;
mod ring_buffer;
/// Function call and return tracing capabilities.
pub mod tracer;

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
extern crate parity_wasm;
extern crate itertools;

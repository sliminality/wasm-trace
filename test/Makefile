FILE = test
MINFILE = $(FILE).min

# wasm function name and arguments
FUNCTION = call_multiple
ARGS = 10

# Compile `test.rs` and execute in Node.
all:
	make build
	make gc
	make wast
	make node

clean:
	rm *.wasm *.wast

# Compile `.rs` to `.wasm` with minimal optimization.
build:
	rustc +nightly --target wasm32-unknown-unknown \
		-C opt-level=1 \
		--crate-type=cdylib $(FILE).rs -o $(FILE).wasm

# Slim down `.wasm` files.
# Requires [wasm-gc](https://github.com/alexcrichton/wasm-gc).
gc:
	wasm-gc $(FILE).wasm -o $(FILE).min.wasm

# Disassemble `.wasm` into `.wast` S-expressions.
# Requires [Binaryen](https://github.com/WebAssembly/binaryen/) .
# Highly recommended to run `make gc` first, otherwise the resulting
# `.wast` is huge..
wast:
	wasm-dis $(MINFILE).wasm -o $(MINFILE).wast

# Execute a `.wasm` file in Node.js.
node:
	node ./bin/runWasm.js $(FILE).wasm $(FUNCTION) $(ARGS)

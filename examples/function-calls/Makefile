FILE = main
MINFILE = $(FILE).min

# wasm function name and arguments
FUNCTION = double_subtract5_add1
ARGS = 10

# Compile `test.rs` and execute in Node.
all:
	make build
	make gc
	make wat
	make node

clean:
	rm -f *.wasm *.wat *.wast

# Compile `.rs` to `.wasm` with minimal optimization.
build:
	rustc +nightly --target wasm32-unknown-unknown \
		-C opt-level=1 \
		--crate-type=cdylib $(FILE).rs -o $(FILE).wasm

# Slim down `.wasm` files.
# Requires [wasm-gc](https://github.com/alexcrichton/wasm-gc).
gc:
	wasm-gc $(FILE).wasm -o $(FILE).min.wasm

# Disassemble `.wasm` into `.wat` S-expressions.
# Requires [Binaryen](https://github.com/WebAssembly/binaryen/) .
# Highly recommended to run `make gc` first, otherwise the resulting
# `.wat` is huge..
wat:
	wasm-dis $(MINFILE).wasm -o $(MINFILE).wat

# Execute a `.wasm` file in Node.js.
node:
	node ../bin/runWasm.js $(FILE).wasm $(FUNCTION) $(ARGS)

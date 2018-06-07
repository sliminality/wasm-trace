EXAMPLE = function-calls
TARGET_DIR = ../../target/wasm32-unknown-unknown/debug/examples

# Modified .wasm binary outputted by main.
OUTPUT = output

# wasm function name and arguments
FUNCTION = do_stuff
ARGS = 4

# Compile `test.rs` and execute in Node.
all:
	make build
	make gc
	make wat
	make node
	cargo run $(EXAMPLE).wasm
	make node EXAMPLE=$(OUTPUT)

clean:
	rm -f *.wasm *.wat *.wast

# Compile `.rs` to `.wasm` with minimal optimization.
build:
	cargo build --example $(EXAMPLE) \
		--target=wasm32-unknown-unknown
	cp $(TARGET_DIR)/$(EXAMPLE).wasm .

# Slim down `.wasm` files.
# Requires [wasm-gc](https://github.com/alexcrichton/wasm-gc).
gc:
	wasm-gc $(EXAMPLE).wasm

# Disassemble `.wasm` into `.wat` S-expressions.
# Requires [Binaryen](https://github.com/WebAssembly/binaryen/) .
# Highly recommended to run `make gc` first, otherwise the resulting
# `.wat` is huge..
wat:
	wasm-dis $(EXAMPLE).wasm -o $(EXAMPLE).wat

# Execute a `.wasm` file in Node.js.
node:
	node ../js/runWasm.js $(EXAMPLE).wasm $(FUNCTION) $(ARGS)

FILE = function-names
MINFILE = $(FILE).min

# wasm function name and arguments
FUNCTION = _Z4add1i
ARGS = 10

# Compile `test.rs` and execute in Node.
all:
	make gc
	make eval
	make wat
	make print

clean:
	rm -f *.wat

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

# Pretty-print a .wat file
print:
	wasm-opt $(MINFILE).wat --print

# Execute a `.wasm` file in Node.js.
eval:
	node ./node/bin/runWasm.js $(FILE).wasm $(FUNCTION) $(ARGS)

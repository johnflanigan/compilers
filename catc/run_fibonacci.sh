echo $1
export OUT_DIR=.
cargo run examples/fibonacci.cat -o examples/fibonacci.s
gcc examples/fibonacci.s src/runtime.c -o examples/fibonacci
./examples/fibonacci

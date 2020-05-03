echo $1
export OUT_DIR=.
cargo run examples/example$1.cat -o examples/example$1.s
gcc examples/example$1.s src/runtime.c -o examples/example$1
./examples/example$1

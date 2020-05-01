export EXAMPLE_NUMBER=7
cargo run examples/example$EXAMPLE_NUMBER.cat -o examples/example$EXAMPLE_NUMBER.s
gcc examples/example$EXAMPLE_NUMBER.s src/runtime.c -o examples/example$EXAMPLE_NUMBER
./examples/example$EXAMPLE_NUMBER

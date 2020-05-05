echo $1
gcc examples/example$1.s src/runtime.c -o examples/example$1
./examples/example$1

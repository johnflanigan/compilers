.globl _main
_main:
    pushq %rbp
    movq %rsp, %rbp
    sub $16, %rsp
    movq 8(%rsi), %rdi
    movq $100, %r12
    movq $0, %r13
    movq $0, %r14
L0:
    cmp %r12, %r14
    je L1
    inc %r14
    add %r14, %r13
    jmp L0
L1:
    movabsq $L2, %rdi
    call _print_string
    movq %r12, %rdi
    call _print_int
    movabsq $L3, %rdi
    call _print_string
    movq %r13, %rdi
    call _print_line_int
    movq $0, %rax
    add $16, %rsp
    popq %rbp
    ret 
L3:
    .string " is "
L2:
    .string "Sum from 1 to "


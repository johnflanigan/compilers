.globl _main
_main:
    pushq %rbp
    movq %rsp, %rbp
    sub $16, %rsp
    movq 8(%rsi), %rdi
    call _atoi
    movq %rax, %rsi
    movabsq $L0, %rdi
    movq $0, %rdx
    movq $0, %rcx
L1:
    cmp %rsi, %rcx
    je L2
    inc %rcx
    add %rcx, %rdx
    jmp L1
L2:
    call _printf
    movq $0, %rax
    add $16, %rsp
    popq %rbp
    ret 
L0:
    .string "Sum from 1 to %d is %d\n"


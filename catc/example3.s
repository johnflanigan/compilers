.globl _main
_main:
    pushq %rbp
    movq %rsp, %rbp
    sub $16, %rsp
    movq %rsp, %r15
    sub $48, %rsp
    movq $-1, %r11
    movq $5, 0(%r15, %r11, 8)
    movq $-2, %r11
    movq $1, 0(%r15, %r11, 8)
    movq $-3, %r11
    movq $4, 0(%r15, %r11, 8)
    movq $-4, %r11
    movq $2, 0(%r15, %r11, 8)
    movq $-5, %r11
    movq $8, 0(%r15, %r11, 8)
    movq $5, %r12
    movq $0, %r13
L0:
    movq %r12, %r8
    dec %r8
    cmp %r13, %r8
    jle L4
    movq $1, %r14
L1:
    movq %r12, %r8
    sub %r13, %r8
    cmp %r14, %r8
    jle L3
    movq %r14, %r9
    neg %r9
    movq %r14, %r10
    neg %r10
    dec %r10
    movq 0(%r15, %r9, 8), %rdi
    movq 0(%r15, %r10, 8), %rsi
    cmp %rdi, %rsi
    jl L5
L2:
    inc %r14
    jmp L1
L3:
    inc %r13
    jmp L0
L4:
    movabsq $L6, %rdi
    call _print_string
    movq $-1, %r11
    movq 0(%r15, %r11, 8), %rdi
    call _print_int
    movabsq $L7, %rdi
    call _print_string
    movq $-2, %r11
    movq 0(%r15, %r11, 8), %rdi
    call _print_int
    movabsq $L7, %rdi
    call _print_string
    movq $-3, %r11
    movq 0(%r15, %r11, 8), %rdi
    call _print_int
    movabsq $L7, %rdi
    call _print_string
    movq $-4, %r11
    movq 0(%r15, %r11, 8), %rdi
    call _print_int
    movabsq $L7, %rdi
    call _print_string
    movq $-5, %r11
    movq 0(%r15, %r11, 8), %rdi
    call _print_line_int
    add $48, %rsp
    movq $0, %rax
    add $16, %rsp
    popq %rbp
    ret 
L5:
    movq %rdi, 0(%r15, %r10, 8)
    movq %rsi, 0(%r15, %r9, 8)
    jmp L2
L6:
    .string "Sorted array: "
L7:
    .string ", "


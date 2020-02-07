        .section        __TEXT,__text,regular,pure_instructions
        .globl  _main
_main:
        pushq   %rbp
        movq    %rsp, %rbp
        subq    $16, %rsp

        movq    %rsp, %rdi

        subq    $40, %rsp

        movq    $1, %r12
        movq    $5, (%rdi, %r12, 8)
        movq    $1, %r12
        movq    $1, (%rdi, %r12, 8)
        movq    $1, %r12
        movq    $4, (%rdi, %r12, 8)
        movq    $1, %r12
        movq    $2, (%rdi, %r12, 8)
        movq    $1, %r12
        movq    $8, (%rdi, %r12, 8)

        movq    $5, %rsi

        movq $0, %rdx

outer:
        movq    $1, %rcx

inner:
        movq    %rcx, %r8
        inc     %r8

        movq    (%rdi, %rcx, 8), %r9

        movq    (%rdi, %r8, 8), %r10

        cmp     %r9, %r10
        jg      swap

continue:
        inc     %rcx

        movq %rsi, %r11
        subq %rdx, %r11

        cmp     %rcx, %r11
        jl      inner

        cmp     %rdx, %rsi
        jle     outer

        movabsq $str1, %rdi
        movq    $1, %r12
        movq    (%rdi, %r12, 8), %rsi
        movq    $2, %r12
        movq    (%rdi, %r12, 8), %rdx
        movq    $3, %r12
        movq    (%rdi, %r12, 8), %rcx
        movq    $4, %r12
        movq    (%rdi, %r12, 8), %r8
        movq    $5, %r12
        movq    (%rdi, %r12, 8), %r9

        callq   _printf

        addq    $40, %rsp       

        movq    $0, %rax
        addq    $16, %rsp
        popq    %rbp
        retq

swap:
        movq    %r10, (%rdi, %rcx, 8)

        movq    %r9, (%rdi, %r8, 8)

        jmp     continue

str1:
        .asciz  "Sorted array: %d, %d,%d, %d, %d\n"

        .section        __TEXT,__text,regular,pure_instructions
        .globl  _main
_main:
        pushq   %rbp
        movq    %rsp, %rbp
        subq    $16, %rsp

        /* Set value to compute summation of */
        movq     $100, %r12
        /* Set value sum is stored in */
        movq    $0, %r13
        /* Set index */
        movq    $0, %r14

loop:
        cmp     %r12, %r14
        je      print
        inc     %r14
        add     %r14, %r13
        jmp     loop

print:
        /* Print first part of string */
        movabsq $str1, %rdi
        callq   _print_string

        /* Print first number */
        movq    %r12, %rdi
        callq   _print_int

        /* Print second part of string */
        movabsq $str2, %rdi
        callq   _print_string

        /* Print second number */
        movq    %r13, %rdi
        callq   _print_line_int

        movq    $0, %rax
        addq    $16, %rsp
        popq    %rbp
        retq
str1:
        .string "Sum from 1 to "
str2:
        .string " is "

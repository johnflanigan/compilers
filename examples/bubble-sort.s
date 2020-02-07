        .section        __TEXT,__text,regular,pure_instructions
        .globl  _main
_main:
        pushq   %rbp
        movq    %rsp, %rbp
        subq    $16, %rsp

        /* Store stack pointer */
        movq    %rsp, %r15
        /* Allocate space for array address and 5 elements */
        subq    $48, %rsp

        /* Initialize array */
        movq    $-1, %r11
        movq    $5, (%r15, %r11, 8)
        movq    $-2, %r11
        movq    $1, (%r15, %r11, 8)
        movq    $-3, %r11
        movq    $4, (%r15, %r11, 8)
        movq    $-4, %r11
        movq    $2, (%r15, %r11, 8)
        movq    $-5, %r11
        movq    $8, (%r15, %r11, 8)

        /* Set n */
        movq    $5, %r12
        /* Set i */
        movq    $0, %r13

outer:
        /* Compute n-1 */
        movq    %r12, %r8
        dec     %r8

        /* If i < n - 1, exit outer loop */
        cmp     %r13, %r8
        jle     exit_outer

        /* Set j */
        movq    $1, %r14

inner:
        /* Compute n-i */
        movq    %r12, %r8
        subq    %r13, %r8

        /* If j < n - i, exit inner loop */   
        cmp     %r14, %r8
        jle     exit_inner

        /* Print j */
        /*
        movabsq $str3, %rdi
        movq    %r14, %rsi
        callq   _printf
        */

        /* Set array indices */
        /* Compute -j */
        movq    %r14, %r9
        neg     %r9
        /* Compute (-j-1) */
        movq    %r14, %r10
        neg     %r10
        dec     %r10

        /* Retrieve array values */
        movq    (%r15, %r9, 8), %rdi
        movq    (%r15, %r10, 8), %rsi

        /* Print values being compared */
        /*
        movabsq $str4, %rdi
        movq    %r9, %rsi
        movq    %r10, %rdx
        callq   _printf
        */

        /* If array[-j] < array[-j-1], swap */
        cmp     %rdi, %rsi
        jl      swap

exit_swap:
        /* Increment j and repeat inner loop */
        inc     %r14
        jmp     inner

exit_inner:
        /* Print i */
        /*
        movabsq $str2, %rdi
        movq    %r13, %rsi
        callq   _printf
        */

        /* Increment i and repeat outer loop */
        inc     %r13
        jmp     outer

exit_outer:
        /* Print sorted array */
        movabsq $str1, %rdi
        movq    $-1, %r11
        movq    (%r15, %r11, 8), %rsi
        movq    $-2, %r11
        movq    (%r15, %r11, 8), %rdx
        movq    $-3, %r11
        movq    (%r15, %r11, 8), %rcx
        movq    $-4, %r11
        movq    (%r15, %r11, 8), %r8
        movq    $-5, %r11
        movq    (%r15, %r11, 8), %r9
        callq   _printf

        /* Deallocate space */
        addq    $48, %rsp  

        /* Boilerplate */
        movq    $0, %rax
        addq    $16, %rsp
        popq    %rbp
        retq

swap:
        /* Swap array[-j] and array[-j-1] */
        movq    %rdi, (%r15, %r10, 8)
        movq    %rsi, (%r15, %r9, 8)
        jmp     exit_swap

str1:
        .asciz  "Sorted array: %d, %d, %d, %d, %d\n"
str2:
        .asciz "i: %d\n"
str3:
        .asciz "j: %d\n"
str4:
        .asciz "array[j]: %d, array[j+1]: %d\n"

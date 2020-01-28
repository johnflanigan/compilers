	.section	__TEXT,__text,regular,pure_instructions
	.globl	_main
_main:
        pushq   %rbp
        movq    %rsp, %rbp
        subq    $16, %rsp

	movq     8(%rsi), %rdi
        call     _atoi
	movq     %rax, %rsi
        movabsq $str1, %rdi
        callq   _printf

        movl    $0, %eax
        addq    $16, %rsp
        popq    %rbp
        retq
str1:
        .asciz  "Argv[1] is %d\n"

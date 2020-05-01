.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$24, %rsp
	movq	-16(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



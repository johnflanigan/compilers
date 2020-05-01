.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$24, %rsp
	movq	$2147483647, -16(%rbp)
	movq	-16(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



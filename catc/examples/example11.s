.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$40, %rsp
	movq	$10, -24(%rbp)
	movq	$11, -32(%rbp)
	movq	-24(%rbp), %rax
	cmp	%rax, -32(%rbp)
	jl	L0
	jmp	L1
L0:
	movq	$1, -16(%rbp)
	jmp	L2
L0:
	movq	$0, -16(%rbp)
L2:
	movq	-16(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



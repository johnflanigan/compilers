.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$40, %rsp
	movq	$10, -32(%rbp)
	movq	$10, -16(%rbp)
	movq	-32(%rbp), %rax
	cmp	%rax, -16(%rbp)
	jle	L0
	jmp	L1
L0:
	movq	$1, -24(%rbp)
	jmp	L2
L0:
	movq	$0, -24(%rbp)
L2:
	movq	-24(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$40, %rsp
	movq	$11, -16(%rbp)
	movq	$10, -24(%rbp)
	movq	-16(%rbp), %rax
	cmp	%rax, -24(%rbp)
	jg	L0
	jmp	L1
L0:
	movq	$1, -32(%rbp)
	jmp	L2
L0:
	movq	$0, -32(%rbp)
L2:
	movq	-32(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



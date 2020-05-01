.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$48, %rsp
	movq	$1, -40(%rbp)
	movq	$13, -32(%rbp)
L0:
	movq	$15, -24(%rbp)
	movq	-32(%rbp), %rax
	cmp	%rax, -24(%rbp)
	je	L1
	jmp	L1
	movq	-32(%rbp), %rax
	movq	%rax, -32(%rbp)
	movq	-32(%rbp), %rax
	movq	%rax, -32(%rbp)
	movq	-40(%rbp), %rax
	add	%rax, -32(%rbp)
	jmp	L0
L1:
	movq	-16(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



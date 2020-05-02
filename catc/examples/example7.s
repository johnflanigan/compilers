.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$56, %rsp
	movq	$1, -16(%rbp)
	movq	$11, -32(%rbp)
	movq	-32(%rbp), %rax
	movq	%rax, -48(%rbp)
	movq	$10, -40(%rbp)
L0:
	movq	-40(%rbp), %rax
	cmp	%rax, -48(%rbp)
	jg	L1
	movq	-48(%rbp), %rax
	movq	%rax, -48(%rbp)
	movq	-48(%rbp), %rax
	movq	%rax, -48(%rbp)
	movq	-16(%rbp), %rax
	add	%rax, -48(%rbp)
	jmp	L0
L1:
	movq	-24(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



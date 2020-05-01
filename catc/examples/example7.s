.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$48, %rsp
	movq	$1, -16(%rbp)
	movq	$11, -40(%rbp)
L0:
	movq	$10, -24(%rbp)
	movq	-40(%rbp), %rax
	cmp	%rax, -24(%rbp)
	je	L1
	movq	-40(%rbp), %rax
	movq	%rax, -40(%rbp)
	movq	-40(%rbp), %rax
	movq	%rax, -40(%rbp)
	movq	-16(%rbp), %rax
	add	%rax, -40(%rbp)
	jmp	L0
L1:
	movq	-32(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$48, %rsp
	movq	$1, -24(%rbp)
	movq	$0, -16(%rbp)
L0:
	movq	$10, -40(%rbp)
	movq	-16(%rbp), %rax
	cmp	%rax, -40(%rbp)
	je	L1
	movq	-16(%rbp), %rax
	movq	%rax, -16(%rbp)
	movq	-16(%rbp), %rax
	movq	%rax, -16(%rbp)
	movq	-24(%rbp), %rax
	add	%rax, -16(%rbp)
	jmp	L0
L1:
	movq	-32(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



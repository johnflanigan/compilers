.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$32, %rsp
	movq	$10, -24(%rbp)
	movq	-24(%rbp), %rdi
	call	L0
	movq	%rax, -32(%rbp)
	movq	-24(%rbp), %rdi
	call	_print_int
	movq	%rax, -16(%rbp)
	movq	-16(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	

L0:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$48, %rsp
	movq	%rdi, -40(%rbp)
	movq	$1, -16(%rbp)
	movq	-40(%rbp), %rax
	movq	$0, %rdx
	sub	-16(%rbp), %rax
	movq	%rax, -24(%rbp)
	movq	-40(%rbp), %rdi
	call	L0
	movq	%rax, -32(%rbp)
	movq	-24(%rbp), %rdi
	call	L0
	movq	%rax, -48(%rbp)
	movq	-48(%rbp), %rax
	movq	$0, %rdx
	add	-32(%rbp), %rax
	movq	%rax, -16(%rbp)
	movq	-16(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	


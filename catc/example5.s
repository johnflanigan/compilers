.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$40, %rsp
	movq	$1, -24(%rbp)
	movq	$1, -40(%rbp)
	movq	-24(%rbp), %rax
	cmp	%rax, -40(%rbp)
	jne	L0
	movq	$L1, -16(%rbp)
	movq	-16(%rbp), %rdi
	call	_print_line_string
	movq	%rax, -32(%rbp)
L0:
	movq	-24(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	

L1:	.string "condition false"


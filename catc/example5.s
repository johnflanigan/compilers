.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$48, %rsp
	movq	$1, -16(%rbp)
	movq	$1, -40(%rbp)
	movq	-16(%rbp), %rax
	cmp	%rax, -40(%rbp)
	jne	L0
	lea	L1(%rip), %rax
	movq	%rax, -32(%rbp)
	movq	-32(%rbp), %rdi
	call	_print_line_string
	movq	%rax, -24(%rbp)
L0:
	movq	-16(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	

L1:	.string "condition false"


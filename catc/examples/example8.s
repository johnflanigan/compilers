.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$56, %rsp
	movq	$2, -16(%rbp)
	movq	$10, -24(%rbp)
	movq	-16(%rbp), %rax
	cmp	%rax, -24(%rbp)
	jl	L1
	jmp	L2
L1:
	movq	$1, -40(%rbp)
	jmp	L3
L1:
	movq	$0, -40(%rbp)
L3:
	movq	$0, -48(%rbp)
	movq	-40(%rbp), %rax
	cmp	%rax, -48(%rbp)
	jne	L0
L0:
	movq	-32(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



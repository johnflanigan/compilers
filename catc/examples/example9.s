.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$64, %rsp
	movq	$2, -16(%rbp)
	movq	$10, -48(%rbp)
	movq	-16(%rbp), %rax
	cmp	%rax, -48(%rbp)
	jl	L3
	jmp	L4
L3:
	movq	$1, -24(%rbp)
	jmp	L5
L3:
	movq	$0, -24(%rbp)
L5:
	movq	$0, -56(%rbp)
	movq	-24(%rbp), %rax
	cmp	%rax, -56(%rbp)
	jne	L0
	movq	-24(%rbp), %rax
	cmp	%rax, -56(%rbp)
	jne	L1
L0:
	movq	$2, -40(%rbp)
	jmp	L2
L1:
	movq	$10, -32(%rbp)
L2:
	movq	-40(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$64, %rsp
	movq	$2, -40(%rbp)
	movq	$10, -56(%rbp)
	movq	-40(%rbp), %rax
	cmp	%rax, -56(%rbp)
	jg	L3
	jmp	L4
L3:
	movq	$1, -16(%rbp)
	jmp	L5
L3:
	movq	$0, -16(%rbp)
L5:
	movq	$0, -24(%rbp)
	movq	-16(%rbp), %rax
	cmp	%rax, -24(%rbp)
	jne	L0
	movq	-16(%rbp), %rax
	cmp	%rax, -24(%rbp)
	jne	L1
L0:
	movq	$2, -32(%rbp)
	jmp	L2
L1:
	movq	$10, -48(%rbp)
L2:
	movq	-32(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



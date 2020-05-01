.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$96, %rsp
	movq	$40, -48(%rbp)
	movq	-48(%rbp), %rax
	movq	%rax, -88(%rbp)
	movq	-88(%rbp), %rax
	movq	%rax, -24(%rbp)
	movq	$50, -64(%rbp)
	movq	-24(%rbp), %rax
	cmp	%rax, -64(%rbp)
	jl	L0
	jmp	L1
L0:
	movq	$1, -32(%rbp)
	jmp	L2
L0:
	movq	$0, -32(%rbp)
L2:
	movq	-88(%rbp), %rax
	movq	%rax, -40(%rbp)
	movq	$10, -56(%rbp)
	movq	-56(%rbp), %rax
	movq	%rax, -40(%rbp)
	movq	$0, -80(%rbp)
	movq	-32(%rbp), %rax
	movq	%rax, -72(%rbp)
	movq	-32(%rbp), %rax
	movq	%rax, -72(%rbp)
	movq	-80(%rbp), %rax
	or	%rax, -72(%rbp)
	movq	-88(%rbp), %rax
	movq	%rax, -16(%rbp)
	movq	-16(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



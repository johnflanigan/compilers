.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$96, %rsp
	movq	$40, -72(%rbp)
	movq	-72(%rbp), %rax
	movq	%rax, -80(%rbp)
	movq	-80(%rbp), %rax
	movq	%rax, -64(%rbp)
	movq	$50, -40(%rbp)
	movq	-64(%rbp), %rax
	cmp	%rax, -40(%rbp)
	jl	L0
	jmp	L1
L0:
	movq	$1, -48(%rbp)
	jmp	L2
L0:
	movq	$0, -48(%rbp)
L2:
	movq	-80(%rbp), %rax
	movq	%rax, -16(%rbp)
	movq	$10, -24(%rbp)
	movq	-24(%rbp), %rax
	movq	%rax, -16(%rbp)
	movq	$0, -32(%rbp)
	movq	-48(%rbp), %rax
	movq	%rax, -56(%rbp)
	movq	-48(%rbp), %rax
	movq	%rax, -56(%rbp)
	movq	-32(%rbp), %rax
	and	%rax, -56(%rbp)
	movq	-80(%rbp), %rax
	movq	%rax, -88(%rbp)
	movq	-88(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



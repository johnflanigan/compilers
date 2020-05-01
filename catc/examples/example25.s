.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$96, %rsp
	movq	$40, -40(%rbp)
	movq	-40(%rbp), %rax
	movq	%rax, -72(%rbp)
	movq	-72(%rbp), %rax
	movq	%rax, -24(%rbp)
	movq	$50, -16(%rbp)
	movq	-24(%rbp), %rax
	cmp	%rax, -16(%rbp)
	jg	L0
	jmp	L1
L0:
	movq	$1, -88(%rbp)
	jmp	L2
L0:
	movq	$0, -88(%rbp)
L2:
	movq	-72(%rbp), %rax
	movq	%rax, -32(%rbp)
	movq	$10, -80(%rbp)
	movq	-80(%rbp), %rax
	movq	%rax, -32(%rbp)
	movq	$0, -48(%rbp)
	movq	-88(%rbp), %rax
	movq	%rax, -56(%rbp)
	movq	-88(%rbp), %rax
	movq	%rax, -56(%rbp)
	movq	-48(%rbp), %rax
	or	%rax, -56(%rbp)
	movq	-72(%rbp), %rax
	movq	%rax, -64(%rbp)
	movq	-64(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



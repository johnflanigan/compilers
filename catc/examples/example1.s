.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$96, %rsp
	movq	$9, -80(%rbp)
	movq	$10, -32(%rbp)
	movq	$10, -48(%rbp)
	movq	-32(%rbp), %rax
	movq	%rax, -56(%rbp)
	movq	-32(%rbp), %rax
	imulq	-48(%rbp)
	movq	%rax, -56(%rbp)
	movq	-80(%rbp), %rax
	movq	%rax, -24(%rbp)
	movq	-80(%rbp), %rax
	movq	%rax, -24(%rbp)
	movq	-56(%rbp), %rax
	add	%rax, -24(%rbp)
	movq	$9, -40(%rbp)
	movq	$10, -72(%rbp)
	movq	-40(%rbp), %rax
	movq	%rax, -16(%rbp)
	movq	-40(%rbp), %rax
	idivq	-72(%rbp)
	movq	%rax, -16(%rbp)
	movq	-24(%rbp), %rax
	movq	%rax, -64(%rbp)
	movq	-24(%rbp), %rax
	movq	%rax, -64(%rbp)
	movq	-16(%rbp), %rax
	sub	%rax, -64(%rbp)
	movq	-64(%rbp), %rax
	movq	%rax, -88(%rbp)
	negq	-88(%rbp)
	movq	-88(%rbp), %rax
	movq %rax, %rdi
	call _print_line_int
	movq	%rbp, %rsp
	popq	%rbp
	ret	



.globl _main
_main:
	pushq	%rbp
	movq	%rsp, %rbp
	sub	$40, %rsp
	jmp	L1
L0:
L1:
	movq	$0, -32(%rbp)
	movq	$0, -16(%rbp)
	movq	-32(%rbp), %rax
	cmp	%rax, -16(%rbp)
	jne	L0
L2:
	movq	-24(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret	



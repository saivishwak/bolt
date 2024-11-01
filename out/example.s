	.text
	.file	"example_module"
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	leaq	.Lmsg(%rip), %rdi
	xorl	%eax, %eax
	callq	printf@PLT
	movl	$10, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.type	.Lmsg,@object                   # @msg
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lmsg:
	.asciz	"Value: 10\n"
	.size	.Lmsg, 11

	.section	".note.GNU-stack","",@progbits

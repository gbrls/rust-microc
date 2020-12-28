	.file	"a.c"
	.intel_syntax noprefix
	.text
	.globl	fn
	.type	fn, @function
fn:
	; Resetting the stack
	push	rbp
	mov	rbp, rsp

	; Allocating space
	sub	rsp, 32

	mov	[rbp-24], rdi
	mov	rax, [rbp-24]

	mov	[rbp-8], rax
	mov	rax, [rbp-8]

	; A pointer to the string is stored in RDI
	mov	rdi, rax
	call	puts@PLT
	nop
	leave
	ret
	.size	fn, .-fn
	.globl	main
	.type	main, @function
main:
	; Resetting the stack
	push	rbp
	mov	rbp, rsp

	; Allocating space
	sub	rsp, 32

	mov	rax, fs:40
	mov	[rbp-8], rax
	xor	eax, eax
	movabs	rax, 2319670658376885588
	mov	[rbp-20], rax

	; If we remove DWORD PTR
	; gcc'll say that the mov command is ambigous
	mov	DWORD PTR [rbp-12], 3350578

	; Load Effective Address
	lea	rax, [rbp-20]
	; di := ax
	mov	rdi, rax
	call	fn
	mov	eax, 0
	mov	rdx, [rbp-8]
	xor	rdx, fs:40
	je	.L4
	call	__stack_chk_fail@PLT
.L4:
	leave
	ret
	.size	main, .-main
	.ident	"GCC: (Ubuntu 9.3.0-17ubuntu1~20.04) 9.3.0"
	.section	.note.GNU-stack,"",@progbits
	.section	.note.gnu.property,"a"
	.align 8
	.long	 1f - 0f
	.long	 4f - 1f
	.long	 5
0:
	.string	 "GNU"
1:
	.align 8
	.long	 0xc0000002
	.long	 3f - 2f
2:
	.long	 0x3
3:
	.align 8
4:

	global _start

	section .text
_start:
	mov rax, 1	; system call for write
	mov rdi, 1
	mov rsi, message
	mov rdx, 13
	syscall
	mov rax, 60	; system call for exit
	xor rdi, rdi
	syscall

	section .data
message:
	db "Hello, World", 12

	; [header]
	global _start

	section .text
_start:
	; [procedure 0]
	mov rax, 1	; system call for write
	mov rdi, 1
	mov rsi, _data_0
	mov rdx, 13
	syscall
	; [procedure 1]
	mov rax, 1	; system call for write
	mov rdi, 1
	mov rsi, _data_1
	mov rdx, 13
	syscall
	; [exit]
	mov rax, 60	; system call for exit
	xor rdi, rdi
	syscall
	; [data]

	section .data
_data_0:
	db "Hello World 1\n", 15
_data_1:
	db "Hello World 2", 13

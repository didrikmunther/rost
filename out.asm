	; [header]
	global main
	extern printf

	section .text
main:
	; [procedure 0]
	push _data_0
	; [procedure 1]
	push 5
	; [procedure 2]
	push 2
	; [procedure 3]
	pop rdx
	pop rsi
	pop rdi
	xor rax, rax
	call printf
	; [exit program]
	ret

	section .data
_data_0:
	db 37, 105, 58, 32, 37, 105, 10, 0

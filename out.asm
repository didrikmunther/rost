	; [header]
	global main
	extern printf

	section .text
main:
	; [procedure 0]
	mov rdi, _data_0
	mov rsi, _data_1
	xor rax, rax
	call printf
	; [exit]
	et	; [exit program]
	; [data]

	section .data
_data_0:
	db 37, 115, 0
_data_1:
	db 72, 101, 108, 108, 111, 9, 87, 111, 114, 108, 100, 32, 49, 10, 0

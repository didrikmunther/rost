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
	db "%s", 10, 0
_data_1:
	db "Hello World 1\n", 10, 0

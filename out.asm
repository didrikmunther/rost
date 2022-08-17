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
	; [procedure 1]
	mov rdi, _data_2
	mov rsi, 5
	xor rax, rax
	call printf
	; [procedure 2]
	mov rdi, _data_3
	mov rsi, _data_4
	xor rax, rax
	call printf
	; [exit]
	ret	; [exit program]
	; [data]

	section .data
_data_0:
	db 37, 115, 0
_data_1:
	db 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 58, 9, 0
_data_2:
	db 37, 105, 0
_data_3:
	db 37, 115, 0
_data_4:
	db 10, 0

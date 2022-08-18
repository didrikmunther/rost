	; [header]
	global main
	extern printf

	section .text
main:
	mov rax, 1
	push rax

	mov rax, 10
	push rax

	mov rax, [rsp+8]
	mov rbx, [rsp]
	add rax, rbx

	; [procedure 0]
	mov rdi, _data_0
	mov rsi, rax
	xor rax, rax
	call printf
	; [exit]

	pop rbx
	pop rbx

	ret	; [exit program]
	; [data]

	section .data
_data_0:
	db "%i", 0
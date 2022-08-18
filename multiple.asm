	; [header]
	global main
	extern printf

	section .text
main:
	; printf("%i %i", 1 + 2 + 3, 4 + 5 + 6)
	push _data_0

	push 1
	push 2
	pop rax
	pop rbx
	add rax, rbx
	push rax

	push 3
	pop rax
	pop rbx
	add rax, rbx
	push rax

	push 4
	push 5
	push 6
	pop rax
	pop rbx
	add rax, rbx
	pop rbx
	add rax, rbx
	push rax

	pop rdx
	pop rsi
	pop rdi

	; [procedure 0]
	; mov rdi, _data_0
	; mov rsi, [rsp+8]
	; mov rdx, [rsp]
	xor rax, rax
	call printf
	; [exit]

	ret	; [exit program]
	; [data]

	section .data
_data_0:
	db "%i, %i", 0
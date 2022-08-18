	; [header]
	global main
	extern printf

	section .text
main:
	mov rax, 5
	push rax ; let a = 5

	mov rax, [rsp] 	;	 a
	add rax, 2		; + 2

	push rax 		; let b = ...

	mov rax, [rsp+8] 	; b
	add rax, [rsp]		; + a
	add rax, 1			; + 1
	push rax			; let c = ...

	; [procedure 0]
	mov rdi, _data_0
	mov rsi, [rsp] 		; c
	xor rax, rax
	call printf
	; [exit]

	pop rbx
	pop rbx
	pop rbx

	ret	; [exit program]
	; [data]

	section .data
_data_0:
	db "%i", 0
	; [header]
	global main
	extern printf

	section .text
main:
	push 5 		; 5 => let a = [rsp]

	mov rax, [rsp] 	;	 a
	push rax

	pop rax
	add rax, 2		; + 2

	push rax 		; let b = [rsp]

	mov rax, [rsp+8] 	; a
	push rax

	mov rbx, [rsp+8]		; b
	push rbx

	pop rax
	pop rbx
	add rax, rbx
	push rax            ; a + b

	push 1				; 1
	pop rax				; 1
	pop rbx				; a + b
	add rax, rbx		; a + b + 1
	push rax

	pop rax

	push rax
	push _data_0

	pop rdi
	pop rsi

	; [procedure 0]
	xor rax, rax
	call printf
	; [exit]

	pop rbx
	pop rbx
	; pop rbx

	ret	; [exit program]
	; [data]

	section .data
_data_0:
	db "%i", 0
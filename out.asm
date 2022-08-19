	global main
	extern printf

	section .text
main:
	push 1
	push 3
	mov rcx, [rsp+8]
	mov rax, rcx
	pop rbx
	add rax, rbx
	push rax
	mov rax, 2
	mov [rsp+8], rax
	mov rcx, [rsp+0]
	push rcx
	push 1
	mov rax, 3
	pop rbx
	add rax, rbx
	pop rbx
	add rax, rbx
	mov [rsp+8], rax
	push _data_0
	push _data_1
	mov rcx, [rsp+24]
	push rcx
	mov rcx, [rsp+24]
	push rcx
	mov rcx, [rsp+24]
	pop rdx
	pop rsi
	pop rdi
	xor rax, rax
	call printf
	pop rax
	pop rax
	pop rax
	ret

	section .data
_data_0:
	db 97, 98, 99, 0
_data_1:
	db 37, 105, 58, 32, 37, 105, 58, 32, 37, 115, 10, 0

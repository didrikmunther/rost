	; [header]
	global main
	extern printf

	section .text
main:
	; [procedure 0]: Push(Int(123))
	push 123
	; [procedure 1]: Comment("Assignment: b, stack: 0")
	; Assignment: b, stack: 0
	; [procedure 2]: Push(Int(2))
	push 2
	; [procedure 3]: Push(StackLocation(0))
	mov rcx, [rsp+8]
	push rcx
	; [procedure 4]: Arithmetic(Add)
	pop rax
	pop rbx
	add rax, rbx
	push rax
	; [procedure 5]: Comment("Assignment: a, stack: 1")
	; Assignment: a, stack: 1
	; [procedure 6]: Push(StackLocation(1))
	mov rcx, [rsp+0]
	push rcx
	; [procedure 7]: Push(Int(2))
	push 2
	; [procedure 8]: Push(StackLocation(1))
	mov rcx, [rsp+16]
	push rcx
	; [procedure 9]: Arithmetic(Add)
	pop rax
	pop rbx
	add rax, rbx
	push rax
	; [procedure 10]: Arithmetic(Add)
	pop rax
	pop rbx
	add rax, rbx
	push rax
	; [procedure 11]: Reassign(1)
	pop rax
	mov [rsp+0], rax
	; [procedure 12]: Push(ByteLocation(0))
	push _data_0
	; [procedure 13]: Push(StackLocation(1))
	mov rcx, [rsp+8]
	push rcx
	; [procedure 14]: SystemCall(SystemCall { identifier: "printf", nargs: 2 })
	pop rsi
	pop rdi
	xor rax, rax
	call printf
	pop rax	; Cleaning stack: 0
	pop rax	; Cleaning stack: 1
	; [exit program]
	ret

	section .data
_data_0:
	db 37, 105, 10, 0

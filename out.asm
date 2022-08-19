	; [header]
	global main
	extern printf

	section .text
main:
	; [procedure 0]: Push(Int(1))
	push 1
	; [procedure 1]: Comment("Assignment: a, stack: 0")
	; Assignment: a, stack: 0
	; [procedure 2]: Push(Int(3))
	push 3
	; [procedure 3]: Push(StackLocation(0))
	mov rcx, [rsp+8]
	push rcx
	; [procedure 4]: Arithmetic(Add)
	pop rax
	pop rbx
	add rax, rbx
	push rax
	; [procedure 5]: Comment("Assignment: b, stack: 1")
	; Assignment: b, stack: 1
	; [procedure 6]: Push(Int(2))
	push 2
	; [procedure 7]: Reassign(0)
	pop rax
	mov [rsp+8], rax
	; [procedure 8]: Push(Int(1))
	push 1
	; [procedure 9]: Push(Int(3))
	push 3
	; [procedure 10]: Arithmetic(Add)
	pop rax
	pop rbx
	add rax, rbx
	push rax
	; [procedure 11]: Reassign(0)
	pop rax
	mov [rsp+8], rax
	; [procedure 12]: Push(ByteLocation(0))
	push _data_0
	; [procedure 13]: Push(StackLocation(0))
	mov rcx, [rsp+16]
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
	db 37, 105, 0

	; [header]
	global main
	extern printf

	section .text
main:
	; [procedure 0]: Push(Int(100))
	push 100
	; [procedure 1]: Comment("Assignment: a, stack: 0")
	; Assignment: a, stack: 0
	; [procedure 2]: Push(Int(10))
	push 10
	; [procedure 3]: Comment("Assignment: b, stack: 1")
	; Assignment: b, stack: 1
	; [procedure 4]: Push(StackLocation(1))
	mov rcx, [rsp+0]
	push rcx
	; [procedure 5]: Push(StackLocation(0))
	mov rcx, [rsp+16]
	push rcx
	; [procedure 6]: Arithmetic(Add)
	pop rax
	pop rbx
	add rax, rbx
	push rax
	; [procedure 7]: Comment("Assignment: c, stack: 2")
	; Assignment: c, stack: 2
	; [procedure 8]: Push(Int(5))
	push 5
	; [procedure 9]: Comment("Assignment: d, stack: 3")
	; Assignment: d, stack: 3
	; [procedure 10]: Push(ByteLocation(0))
	push _data_0
	; [procedure 11]: Push(Int(1))
	push 1
	; [procedure 12]: Push(StackLocation(2))
	mov rcx, [rsp+24]
	push rcx
	; [procedure 13]: Arithmetic(Add)
	pop rax
	pop rbx
	add rax, rbx
	push rax
	; [procedure 14]: SystemCall(SystemCall { identifier: "printf", nargs: 2 })
	pop rsi
	pop rdi
	xor rax, rax
	call printf
	pop rax	; Cleaning stack: 0
	pop rax	; Cleaning stack: 1
	pop rax	; Cleaning stack: 2
	pop rax	; Cleaning stack: 3
	; [exit program]
	ret

	section .data
_data_0:
	db 37, 105, 0

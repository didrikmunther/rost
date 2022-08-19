	; [header]
	global main
	extern printf

	section .text
	; [procedure 0]: Push(Int(1))
main:
	; [procedure 1]: Comment("Assignment: a, stack: 0")
	; Assignment: a, stack: 0
	; [procedure 2]: Push(Int(3))
	push 1
	; [procedure 3]: Push(StackLocation(0))
	push 3
	mov rcx, [rsp+8]
	; [procedure 4]: Arithmetic(Add)
	; Optimized: removed push / pop, added mov
	mov rax, rcx
	pop rbx
	add rax, rbx
	; [procedure 5]: Comment("Assignment: b, stack: 1")
	; Assignment: b, stack: 1
	; [procedure 6]: Push(Int(2))
	push rax
	; [procedure 7]: Reassign(0)
	; Optimized: removed push / pop, added mov
	mov rax, 2
	; [procedure 8]: Push(StackLocation(1))
	mov [rsp+8], rax
	mov rcx, [rsp+0]
	; [procedure 9]: Push(Int(1))
	push rcx
	; [procedure 10]: Push(Int(3))
	push 1
	; [procedure 11]: Arithmetic(Add)
	; Optimized: removed push / pop, added mov
	mov rax, 3
	pop rbx
	add rax, rbx
	; [procedure 12]: Arithmetic(Add)
	; Optimized: removed push / pop
	pop rbx
	add rax, rbx
	; [procedure 13]: Reassign(0)
	; Optimized: removed push / pop
	; [procedure 14]: Push(ByteLocation(0))
	mov [rsp+8], rax
	; [procedure 15]: Comment("Assignment: c, stack: 2")
	; Assignment: c, stack: 2
	; [procedure 16]: Push(ByteLocation(1))
	push _data_0
	; [procedure 17]: Push(StackLocation(0))
	push _data_1
	mov rcx, [rsp+24]
	; [procedure 18]: Push(StackLocation(1))
	push rcx
	mov rcx, [rsp+24]
	; [procedure 19]: Push(StackLocation(2))
	push rcx
	mov rcx, [rsp+24]
	; [procedure 20]: SystemCall(SystemCall { identifier: "printf", nargs: 4 })
	; Optimized: removed push / pop
	pop rdx
	pop rsi
	pop rdi
	xor rax, rax
	call printf
	; Cleaning stack variable: a
	pop rax
	; Cleaning stack variable: b
	pop rax
	; Cleaning stack variable: c
	; [exit program]
	pop rax
	ret

	section .data
_data_0:
	db 97, 98, 99, 0
_data_1:
	db 37, 105, 58, 32, 37, 105, 58, 32, 37, 115, 10, 0

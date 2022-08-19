	; [header]
	global main
	extern printf

	section .text
main:
	; [procedure 0]: Push(Int(2))
	push 2
	; [procedure 1]: Push(Int(5))
	push 5
	; [procedure 2]: Arithmetic(Add)
	pop rax
	pop rbx
	add rax, rbx
	push rax
	; [procedure 3]: Comment("Assignment: a, stack: 0")
	; Assignment: a, stack: 0
	; [procedure 4]: Push(ByteLocation(0))
	push _data_0
	; [procedure 5]: Push(StackLocation(0))
	mov rcx, [rsp+8]
	push rcx
	; [procedure 6]: SystemCall(SystemCall { identifier: "printf", nargs: 2 })
	pop rsi
	pop rdi
	xor rax, rax
	call printf
	pop rax	; Cleaning stack: 0
	; [exit program]
	ret

	section .data
_data_0:
	db 37, 105, 0

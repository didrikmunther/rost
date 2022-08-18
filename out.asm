	; [header]
	global main
	extern printf

	section .text
main:
	; [procedure 0]: Push(ByteLocation(0))
	push _data_0
	; [procedure 1]: Push(Int(1))
	push 1
	; [procedure 2]: Push(Int(2))
	push 2
	; [procedure 3]: Arithmetic(Add)
	pop rax
	pop rbx
	add rax, rbx
	push rax
	; [procedure 4]: Push(Int(3))
	push 3
	; [procedure 5]: Arithmetic(Add)
	pop rax
	pop rbx
	add rax, rbx
	push rax
	; [procedure 6]: Push(Int(4))
	push 4
	; [procedure 7]: Push(Int(5))
	push 5
	; [procedure 8]: Arithmetic(Add)
	pop rax
	pop rbx
	add rax, rbx
	push rax
	; [procedure 9]: Push(Int(6))
	push 6
	; [procedure 10]: Arithmetic(Add)
	pop rax
	pop rbx
	add rax, rbx
	push rax
	; [procedure 11]: SystemCall(SystemCall { identifier: "printf", nargs: 3 })
	pop rdx
	pop rsi
	pop rdi
	xor rax, rax
	call printf
	; [procedure 12]: Push(ByteLocation(1))
	push _data_1
	; [procedure 13]: SystemCall(SystemCall { identifier: "printf", nargs: 1 })
	pop rdi
	xor rax, rax
	call printf
	; [exit program]
	ret

	section .data
_data_0:
	db 37, 105, 32, 37, 105, 10, 0
_data_1:
	db 72, 101, 106, 0

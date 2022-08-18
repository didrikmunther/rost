	; [header]
	global main
	extern printf

	section .text
main:
	; [procedure 0]: Push(ByteLocation(0))
	push _data_0
	; [procedure 1]: SystemCall(SystemCall { identifier: "printf", nargs: 1 })
	pop rdi
	xor rax, rax
	call printf
	; [procedure 2]: Push(ByteLocation(1))
	push _data_1
	; [procedure 3]: Push(Int(1))
	push 1
	; [procedure 4]: Push(Int(2))
	push 2
	; [procedure 5]: Push(Int(3))
	push 3
	; [procedure 6]: Push(Int(4))
	push 4
	; [procedure 7]: Push(Int(5))
	push 5
	; [procedure 8]: SystemCall(SystemCall { identifier: "printf", nargs: 6 })
	pop r9
	pop r8
	pop rcx
	pop rdx
	pop rsi
	pop rdi
	xor rax, rax
	call printf
	; [exit program]
	ret

	section .data
_data_0:
	db 72, 101, 106, 10, 0
_data_1:
	db 37, 105, 32, 37, 105, 32, 37, 105, 32, 37, 105, 32, 37, 105, 10, 0

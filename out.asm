	; [header]
	global main
	extern printf

	section .text
	; [procedure 0]: Push(Int(1))
main:
	; [procedure 1]: Comment("Assignment: a, stack: 0")
	; Assignment: a, stack: 0
	; [procedure 2]: Push(Int(11))
	push 1
	; [procedure 3]: Reassign(0)
	; Optimized: removed push / pop, added mov
	mov rax, 11
	; [procedure 4]: Push(ByteLocation(0))
	mov [rsp+0], rax
	; [procedure 5]: Push(StackLocation(0))
	push _data_0
	mov rcx, [rsp+8]
	; [procedure 6]: SystemCall(SystemCall { identifier: "printf", nargs: 2 })
	; Optimized: removed push / pop, added mov
	mov rsi, rcx
	pop rdi
	xor rax, rax
	call printf
	; Cleaning stack variable: a
	; [exit program]
	pop rax
	ret

	section .data
_data_0:
	db 37, 105, 10, 0

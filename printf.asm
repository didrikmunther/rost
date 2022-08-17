; ----------------------------------------------------------------------------------------
; Writes "Hola, mundo" to the console using a C library. Runs on Linux.
;
;     nasm -felf64 hola.asm && gcc hola.o && ./a.out
; ----------------------------------------------------------------------------------------

          global    main
          extern  printf

          section   .text
main:                                       ; This is called by the C library startup code

        mov     rdi, format             ; set 1st parameter (format)
        mov     rsi, 5                  ; set 2nd parameter (current_number)
        xor     rax, rax                ; because printf is varargs
        ; Stack is already aligned because we pushed three 8 byte registers
        call    printf                  ; printf(format, current_number)
        ret                               ; Return from main back into C library wrapper
        
        section   .data
format:
          db "%i", 10, 0        ; Note strings must be terminated with 0 in C
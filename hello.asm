          global    main
          extern  printf

          section   .text
main: 
        ; Print
        mov       rax, 1                  ; system call for write
        mov       rdi, 1                  ; file handle 1 is stdout
        mov       rsi, message            ; address of string to output
        mov       rdx, 13                 ; number of bytes
        syscall                           ; invoke operating system to do the write

        ; printf
        mov     rdi, format
        mov     rsi, message
        call    [rel printf wrt ..got]

        ; Exit
        mov       rax, 60                 ; system call for exit
        xor       rdi, rdi                ; exit code 0
        syscall                           ; invoke operating system to exit



        section   .data
message:
        db        "Hello, World", 10      ; note the newline at the end
format:
        db "%s extra", 10, 0
(module
    ;; Imports from JavaScript namespace
    (import  "imports"  "print" (func  $print (param  i32  i32))) ;; Import log function
    (import  "js"  "mem" (memory  1)) ;; Import 1 page of memory (54kb)
    
    ;; Data section of our module
    (data (i32.const 0) "Hello World from WebAssembly!\n")
    (data (i32.const 64) "Another string here.\n")
    
    ;; Function declaration: Exported as helloWorld(), no arguments
    (func (export  "main")
        i32.const 0  ;; pass offset 0 to log
        i32.const 30  ;; pass length 29 to log (strlen of sample text)
        call  $print
        i32.const 0  ;; pass offset 0 to log
        i32.const 30  ;; pass length 29 to log (strlen of sample text)
        call  $print
        )
;; boilerplate_entry.wat begin

(module
    ;; Imports from JavaScript namespace
    (import  "imports"  "print" (func  $print (param  i32))) ;; Import print function, print string
    (import  "imports"  "printi" (func  $printi (param  i32))) ;; Import printi function, print integer
    (import  "js"  "mem" (memory  1)) ;; Import 1 page of memory (54kb)

    ;; boilerplate_entry.wat end
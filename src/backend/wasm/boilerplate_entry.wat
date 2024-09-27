;; boilerplate_entry.wat begin

(module
    ;; Imports from JavaScript namespace
    (import  "imports"  "print_raw" (func  $print_raw (param  i32  i32))) ;; Import log function
    (import  "js"  "mem" (memory  1)) ;; Import 1 page of memory (54kb)

;; boilerplate_entry.wat end
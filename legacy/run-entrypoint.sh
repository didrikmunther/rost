#!/bin/bash

(
	nasm -felf64 out.asm &&
	gcc -no-pie -fno-pie out.o &&
	rm ./out.o &&
	./a.out "$@" &&
	rm ./a.out
) || rm ./a.out
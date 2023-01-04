FROM ubuntu:latest

RUN apt update
RUN apt install -y nasm binutils time gcc

WORKDIR /app

CMD (nasm -felf64 out.asm && \
	gcc -no-pie -fno-pie out.o && \
	rm ./out.o && \
	./a.out && \
	rm ./a.out) || \
	rm ./a.out
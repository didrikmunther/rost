FROM rust:latest

RUN apt update
RUN apt install -y nasm binutils

WORKDIR /app

CMD nasm -felf64 out.asm && \
	gcc -no-pie -fno-pie out.o && \
	rm ./out.o && \
	./a.out
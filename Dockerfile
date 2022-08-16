FROM ubuntu:20.04

RUN apt update
RUN apt install -y nasm binutils

WORKDIR /app
COPY . .
RUN nasm -felf64 out.asm
RUN ld out.o

CMD ./a.out
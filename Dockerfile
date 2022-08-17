FROM rust:latest

RUN apt update
RUN apt install -y nasm binutils

WORKDIR /app
COPY . .
RUN nasm -felf64 out.asm && gcc -no-pie -fno-pie out.o

CMD ./a.out
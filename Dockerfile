FROM rust:latest

RUN apt update
RUN apt install -y nasm binutils

WORKDIR /app
COPY . .
RUN nasm -felf64 fib.asm && gcc -no-pie -fno-pie fib.o

CMD ./a.out
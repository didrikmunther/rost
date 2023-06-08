FROM ubuntu:latest

RUN apt update
RUN apt install -y nasm binutils time gcc

WORKDIR /app

ENTRYPOINT ["./run-entrypoint.sh"]
FROM --platform=linux/amd64 ubuntu:latest

RUN apt update && \
	apt install -y nasm binutils time gcc && \
	apt clean

WORKDIR /app

ENTRYPOINT ["./run-entrypoint.sh"]
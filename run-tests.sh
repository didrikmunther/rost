#!/bin/bash

echo "" > tests.log

cargo build || exit

shopt -s nullglob
for file in ./tests/*.in; do
	echo "[Running test ${file}]" | tee -a tests.log
	name=$(echo $file | sed 's/.*\///;s/\.in//')
	./target/debug/rost ./tests/${name}.in "$@" >> tests.log || (echo "Compiler error for test ${name}, check logs" | tee -a tests.log || exit)
	output=$(docker run -v $(PWD):/app rost)
	expected=$(cat ./tests/${name}.out)
	if [ "$output" != "$expected" ]; then
		echo "Got unexpected output for test ${name}. Got:" | tee -a tests.log
		echo "------------" | tee -a tests.log
		echo "${output}" | tee -a tests.log
		echo "------------" | tee -a tests.log
		echo "Expected:" | tee -a tests.log
		echo "------------" | tee -a tests.log
		echo "${expected}" | tee -a tests.log
		echo "------------" | tee -a tests.log
		exit
	fi
	echo "[Tests pass]" | tee -a tests.log
	echo "==========" >> tests.log
done

echo "[All tests pass]" | tee -a tests.log
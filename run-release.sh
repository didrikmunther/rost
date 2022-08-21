#!/bin/bash

cargo build --release && ./target/release/rost "$@" && (docker run -v $(PWD):/app rost || rm ./a.out)
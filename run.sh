#!/bin/bash

cargo build &&
./target/debug/rost "$@" &&
docker run -v $(PWD):/app rost
#!/bin/bash

cargo build && ./target/debug/rost "$@" && ./run_docker.sh
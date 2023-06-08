#!/bin/bash

# Run the rost compiler, forwarding arguments

cargo build &&
./target/debug/rost "$@"
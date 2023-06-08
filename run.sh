#!/bin/bash

# Run the `out.asm` file

docker run -v $(PWD):/app rost "$@"
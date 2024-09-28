#!/bin/bash

wat2wasm build/out.wat -o build/out.wasm && \
node ./rost/src/backend/wasm/orchestrator.js build/out.wasm

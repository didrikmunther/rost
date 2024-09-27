#!/bin/bash

wat2wasm build/out.wat -o build/out.wasm && \
node ./src/backend/wasm/orchestrator.js build/out.wasm

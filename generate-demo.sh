#!/bin/bash

cd rost_wasm &&
wasm-pack build &&
cd ..
rm rost_wasm/pkg/.gitignore rost_wasm/pkg/package.json &&
cp -r rost_wasm/pkg/* demo/src/rost/
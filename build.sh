#!/bin/bash

set -euo pipefail

cd wasm-bindgen && cargo build -p wasm-bindgen-cli
cd -

cargo build --target wasm32-unknown-unknown
./wasm-bindgen/target/debug/wasm-bindgen target/wasm32-unknown-unknown/debug/rustworker.wasm --target module --out-dir build
echo "import $(tail -c +15 build/rustworker.js)" > build/rustworker.js

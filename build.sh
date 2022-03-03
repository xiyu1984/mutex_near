#!/bin/bash
set -e
cd "`dirname $0`"
cargo build --all --target wasm32-unknown-unknown --release
mkdir -p ./res && cp target/wasm32-unknown-unknown/release/*.wasm ./res/


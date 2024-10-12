#!/bin/bash
set -e

cd "$(dirname $0)"

TARGET="$(pwd)/../../target"

rustup target add wasm32-unknown-unknown

cargo build --target wasm32-unknown-unknown --profile app-release

mkdir -p res

cp $TARGET/wasm32-unknown-unknown/app-release/car-sharing.wasm ./res/

#install and use wasm-opt for wasm file size optimization
if command -v wasm-opt > /dev/null; then
  wasm-opt -Oz ./res/car-sharing.wasm -o ./res/car-sharing.wasm
fi
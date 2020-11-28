#!/bin/sh
set -e
# Build
case "$1" in
    r*)
        wasm-pack build --target web --release -- --features wasm
    ;;
    *)
        wasm-pack build --target web --dev -- --features wasm
    ;;
esac
# allow publish
rm ./pkg/package.json
rm ./pkg/.gitignore
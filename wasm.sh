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
cp ./pkg/aoc.js ./pkg/aoc.js.bk
# Do post-bindgen
# ugly
sed -i '/typeof .* == '\''function'\'' ? .* : notDefined('\''.*'\'');/{s! == '\''function'\'' ? ! == '\''function'\'' ? function(){ !g}' ./pkg/aoc.js
sed -i '/typeof .* == '\''function'\'' ? .* : notDefined('\''.*'\'');/{s! : notDefined('\''!(); } : notDefined('\''!g}' ./pkg/aoc.js
# What's changed  in postgen
diff ./pkg/aoc.js ./pkg/aoc.js.bk
# allow publish
rm ./pkg/package.json
rm ./pkg/.gitignore
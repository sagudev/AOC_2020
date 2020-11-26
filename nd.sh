#!/bin/bash
if [ $# -eq 0 ]; then
    echo "No arguments provided"
    exit 1
fi
if [ -f "./data/data$1" ]; then
    echo "./data/data$1 exists."
    exit 1
fi
touch ./data/data$1
printf -v t1 %q "[[bin]]"
printf -v t2 %q "name = \"day$1\""
printf -v t3 %q "path = \"src/bin/day$1.rs\""
sed -i "/path = \"src\/bin\/dayx.rs/a \\\n$t1\n$t2\n$t3" Cargo.toml
touch ./src/bin/day$1.rs
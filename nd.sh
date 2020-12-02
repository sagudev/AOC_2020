#!/bin/bash
if [ $# -eq 0 ]; then
    echo "Provide day number plz"
    exit 1
fi
# do not run same day twice
if [ -f "./data/data$1" ]; then
    echo "./data/data$1 exists."
    exit 1
fi
p=$(expr $1 - 1)
# create new empty data file
touch ./data/data$1
printf -v t1 %q "[[bin]]"
printf -v t2 %q "name = \"day$1\""
printf -v t3 %q "path = \"src/bin/day$1.rs\""
# add new bin to Cargo.toml
sed -i "/path = \"src\/bin\/dayx.rs/a \\\n$t1\n$t2\n$t3" Cargo.toml
sed -i "s/default-run = \"dayx\"/default-run = \"day$1\"/" Cargo.toml
# use template for new day
cp ./src/bin/dayx.rs ./src/bin/day$1.rs
# replace old datapath
sed -i "s|./data/datax|./data/data$1|g" ./src/bin/day$1.rs